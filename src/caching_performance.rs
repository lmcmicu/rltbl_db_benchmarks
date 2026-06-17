use anyhow::Result;
use async_trait::async_trait;
use rand::{
    SeedableRng as _,
    distr::{Distribution as _, Uniform},
    rngs::StdRng,
};
use rlt::{BenchSuite, IterInfo, IterReport, Status, cli::BenchCli};
use rltbl_db::{
    any::AnyPool,
    core::{CachingStrategy, DbQuery},
    db_kind::DbKind,
    memory::clear_meta_cache,
};
//use serde::{Deserialize, Serialize};
use std::{str::FromStr, time::Instant};

#[derive(Clone)]
pub(crate) struct CachingPerformance {
    seed: i64,
    kind: DbKind,
    strategy: CachingStrategy,
    tables: Vec<String>,
    edit_rate: usize,
}

#[async_trait]
impl BenchSuite for CachingPerformance {
    type WorkerState = AnyPool;

    // The comment below is from the source code for the trait in rlt, but I think what it
    // actually does is initialize the state for all of the workers.
    // That said, maybe what needs to be done to get a per-worker state is to somehow
    // use the worker_id.
    // Initialize the state for a worker
    async fn state(&self, _worker_id: u32) -> Result<Self::WorkerState> {
        let pool = {
            let url = match self.kind {
                DbKind::SQLite => ":memory:",
                DbKind::PostgreSQL => "postgresql:///rltbl_db",
            };
            AnyPool::connect(url).await.unwrap()
        };
        Ok(pool)
    }

    // The comment below is from the source code for the trait in rlt, but I think what it
    // actually does is to run the setup procedure for all of the workers (as judged by the
    // number of rows observed in each of the four tables once the test is running), i.e.,
    // before any of them run.
    // That said, maybe what needs to be done to get a per-worker setup is to somehow
    // use the worker_id.
    // Setup procedure before each worker starts.
    async fn setup(&mut self, state: &mut Self::WorkerState, _worker_id: u32) -> Result<()> {
        clear_meta_cache().unwrap();
        self.tables = vec![
            "alpha".to_string(),
            "beta".to_string(),
            "gamma".to_string(),
            "delta".to_string(),
        ];
        for table in &self.tables {
            state.drop_table(table).await.unwrap();
            state.drop_view(&format!("{table}_view")).await.unwrap();
            state
                .execute(&format!("CREATE TABLE {table} ( foo INT, bar INT )"), ())
                .await
                .unwrap();
            state
                .execute(
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
            state
                .execute(
                    &format!("INSERT INTO {table} (foo, bar) VALUES {}", values),
                    (),
                )
                .await
                .unwrap();
        }
        state.set_cache_aware_query(true);
        state.set_caching_strategy(&self.strategy);
        Ok(())
    }

    // The comment below is from the source code for the trait in rlt, but I think what it
    // actually does is to run the teardown procedure for all of the workers, i.e., after they
    // are all done.
    // That said, maybe what needs to be done to get a per-worker teardown is to somehow
    // use the worker_id.
    // Teardown procedure after each worker finishes.
    async fn teardown(self, state: Self::WorkerState, _info: IterInfo) -> Result<()> {
        for table in &self.tables {
            state.drop_table(table).await.unwrap();
        }
        Ok(())
    }

    async fn bench(&mut self, state: &mut Self::WorkerState, _: &IterInfo) -> Result<IterReport> {
        let start = Instant::now();

        let select_table = self.random_table();
        state
            .cache(
                &format!("SELECT foo, SUM(bar) FROM {select_table}_view GROUP BY foo ORDER BY foo"),
                (),
            )
            .await
            .unwrap();
        if self.edit_rate != 0 && self.random_between(0, self.edit_rate) == 0 {
            let table_to_edit = self.random_table();
            state
                .execute(
                    &format!("INSERT INTO {table_to_edit} (foo) VALUES (1), (1)"),
                    (),
                )
                .await
                .unwrap();
        }

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
        _totals_file: &str,
        seed: i64,
    ) {
        println!(
            "Caching Performance Test - Starting test with \
             db kind '{kind}' and strategy '{strategy}'.",
        );

        rlt::cli::run(
            bench.clone(),
            CachingPerformance {
                seed: seed,
                kind: DbKind::from_str(&kind).expect("Error reading database kind"),
                strategy: CachingStrategy::from_str(&strategy).unwrap(),
                tables: vec![],
                edit_rate: edit_rate,
            },
        )
        .await
        .unwrap();
    }
}
