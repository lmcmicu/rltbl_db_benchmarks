use anyhow::Result;
use async_trait::async_trait;
use rand::{
    SeedableRng as _,
    distr::{Distribution as _, Uniform},
    rngs::StdRng,
};
use rlt::{IterInfo, IterReport, StatelessBenchSuite, Status, cli::BenchCli};
use rltbl_db::{
    any::AnyPool,
    core::{CachingStrategy, DbQuery},
    db_kind::DbKind,
};
use std::{str::FromStr, time::Instant};

#[derive(Clone)]
pub(crate) struct CachingPerformance {
    seed: i64,
    pool: AnyPool,
    tables: Vec<String>,
    edit_rate: usize,
}

// Useful struct for validating the total times of the caching performance tests.
struct CachingTotalsBaselines {
    sqlite_none: u64,
    postgresql_none: u64,
    sqlite_truncate_all: u64,
    postgresql_truncate_all: u64,
    sqlite_truncate: u64,
    postgresql_truncate: u64,
    sqlite_trigger: u64,
    postgresql_trigger: u64,
    sqlite_memory: u64,
    postgresql_memory: u64,
}

impl CachingTotalsBaselines {
    fn compare_with(&self, kind: &DbKind, strategy: &CachingStrategy, elapsed: u64) {
        match strategy {
            CachingStrategy::None => match kind {
                DbKind::SQLite if elapsed > self.sqlite_none => {
                    panic!("Took longer than {}s.", self.sqlite_none);
                }
                DbKind::PostgreSQL if elapsed > self.postgresql_none => {
                    panic!("Took longer than {}s.", self.postgresql_none);
                }
                _ => (),
            },
            CachingStrategy::TruncateAll => match kind {
                DbKind::SQLite if elapsed > self.sqlite_truncate_all => {
                    panic!("Took longer than {}s.", self.sqlite_truncate);
                }
                DbKind::PostgreSQL if elapsed > self.postgresql_truncate_all => {
                    panic!("Took longer than {}s.", self.postgresql_truncate);
                }
                _ => (),
            },
            CachingStrategy::Truncate => match kind {
                DbKind::SQLite if elapsed > self.sqlite_truncate => {
                    panic!("Took longer than {}s.", self.sqlite_truncate);
                }
                DbKind::PostgreSQL if elapsed > self.postgresql_truncate => {
                    panic!("Took longer than {}s.", self.postgresql_truncate);
                }
                _ => (),
            },
            CachingStrategy::Trigger => match kind {
                DbKind::SQLite if elapsed > self.sqlite_trigger => {
                    panic!("Took longer than {}s.", self.sqlite_trigger);
                }
                DbKind::PostgreSQL if elapsed > self.postgresql_trigger => {
                    panic!("Took longer than {}s.", self.postgresql_trigger);
                }
                _ => (),
            },
            CachingStrategy::Memory(_) => match kind {
                DbKind::SQLite if elapsed > self.sqlite_memory => {
                    panic!("Took longer than {}s.", self.sqlite_memory);
                }
                DbKind::PostgreSQL if elapsed > self.postgresql_memory => {
                    panic!("Took longer than {}s.", self.postgresql_memory);
                }
                _ => (),
            },
        }
    }
}

#[async_trait]
impl StatelessBenchSuite for CachingPerformance {
    async fn bench(&mut self, _: &IterInfo) -> Result<IterReport> {
        let start = Instant::now();
        self.perform_caching_detail().await;
        let duration = start.elapsed();
        Ok(IterReport {
            duration,
            status: Status::success(0),
            // Not used:
            items: 0,
            bytes: 0,
        })
    }
}

impl CachingPerformance {
    fn random_between(&mut self, min: usize, max: usize) -> usize {
        let between = Uniform::try_from(min..max).unwrap();
        let mut rng = if self.seed < 0 {
            StdRng::from_rng(&mut rand::rng())
        } else {
            self.seed += 10;
            StdRng::seed_from_u64(self.seed as u64)
        };
        between.sample(&mut rng)
    }

    fn random_table<'a>(&mut self) -> String {
        let index = self.random_between(0, self.tables.len());
        self.tables[index].to_string()
    }

    pub(crate) async fn perform_caching(
        kind: &str,
        bench: &BenchCli,
        strategy: &str,
        edit_rate: usize,
        seed: i64,
    ) {
        let mut pool = {
            let kind = DbKind::from_str(&kind).expect("Error reading database kind");
            let url = match kind {
                DbKind::SQLite => ":memory:",
                DbKind::PostgreSQL => "postgresql:///rltbl_db",
            };
            AnyPool::connect(url).await.unwrap()
        };
        let tables_to_choose_from = ["alpha", "beta", "gamma", "delta"];
        for table in &tables_to_choose_from {
            pool.drop_table(table).await.unwrap();
            pool.drop_view(&format!("{table}_view")).await.unwrap();
            pool.execute(&format!("CREATE TABLE {table} ( foo INT, bar INT )"), ())
                .await
                .unwrap();
            pool.execute(
                &format!("CREATE VIEW {table}_view AS SELECT * FROM {table}"),
                (),
            )
            .await
            .unwrap();

            // Add a few tens of thousands of values to the table:
            let mut values = vec![];
            for i in 0..5 {
                for j in 0..30000 {
                    values.push(format!("({i}, {j})"));
                }
            }
            let values = values.join(", ");
            pool.execute(
                &format!("INSERT INTO {table} (foo, bar) VALUES {}", values),
                (),
            )
            .await
            .unwrap();
        }
        pool.set_cache_aware_query(true);
        pool.set_caching_strategy(&CachingStrategy::from_str(&strategy).unwrap());

        println!(
            "Caching Performance Test - Starting test with db kind '{}' and strategy '{}'.",
            pool.kind(),
            pool.get_caching_strategy()
        );

        // Mark the start time of the test:
        let now = Instant::now();

        // Run the test:
        rlt::cli::run(
            bench.clone(),
            CachingPerformance {
                seed: seed,
                pool: pool.clone(),
                tables: tables_to_choose_from
                    .clone()
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>(),
                edit_rate: edit_rate,
            },
        )
        .await
        .unwrap();

        // Check that the overall running time is no longer than the baselines defined below:
        let elapsed = now.elapsed().as_secs();

        // TODO: Read these values from a (JSON?) file, and make it possible to re-save.
        let baselines = CachingTotalsBaselines {
            sqlite_none: 250,
            postgresql_none: 150,
            sqlite_truncate_all: 75,
            postgresql_truncate_all: 65,
            sqlite_truncate: 65,
            postgresql_truncate: 55,
            sqlite_trigger: 20,
            postgresql_trigger: 20,
            sqlite_memory: 20,
            postgresql_memory: 20,
        };

        println!("Completed after {elapsed}s\n");
        baselines.compare_with(&pool.kind(), &pool.get_caching_strategy(), elapsed);

        // Clean up:
        for table in &tables_to_choose_from {
            pool.drop_table(table).await.unwrap();
        }
    }

    async fn perform_caching_detail(&mut self) {
        let select_table = self.random_table();
        self.pool
            .cache(
                &format!("SELECT foo, SUM(bar) FROM {select_table}_view GROUP BY foo ORDER BY foo"),
                (),
            )
            .await
            .unwrap();
        if self.edit_rate != 0 && self.random_between(0, self.edit_rate) == 0 {
            let table_to_edit = self.random_table();
            self.pool
                .execute(
                    &format!("INSERT INTO {table_to_edit} (foo) VALUES (1), (1)"),
                    (),
                )
                .await
                .unwrap();
        }
    }
}
