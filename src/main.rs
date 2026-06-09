use clap::{Parser, Subcommand};
use rlt::cli::BenchCli;
use rltbl_db::{
    any::AnyPool,
    core::{CachingStrategy, DbQuery},
    db_kind::DbKind,
};
use std::{str::FromStr, time::Instant};

mod caching_performance;
use caching_performance::CachingPerformance;

#[derive(Parser, Clone)]
struct Opts {
    #[clap(long, default_value = "-1")]
    seed: i64,

    #[clap(subcommand)]
    command: Subcommands,

    #[command(flatten)]
    bench: BenchCli,
}

#[derive(Clone, Subcommand)]
enum Subcommands {
    Caching {
        #[clap(default_value = "sqlite")]
        kind: String,

        #[clap(default_value = "none")]
        strategy: String,

        #[clap(long, default_value = "25")]
        edit_rate: usize,
    },
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

async fn caching_performance(opts: &Opts) {
    let mut seed = opts.seed;
    let (kind, strategy, edit_rate) = match &opts.command {
        Subcommands::Caching {
            kind,
            strategy,
            edit_rate,
        } => (kind, strategy, edit_rate),
    };
    let mut pool = {
        let kind = DbKind::from_str(&kind.as_str()).expect("Error reading database kind");
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
            for j in 0..CachingPerformance::random_between(34000, 35000, &mut seed) {
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
        opts.bench.clone(),
        CachingPerformance {
            seed: seed,
            pool: pool.clone(),
            tables: tables_to_choose_from
                .clone()
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>(),
            edit_rate: *edit_rate,
        },
    )
    .await
    .unwrap();

    // Check that the overall running time is no longer than the baselines defined below:
    let elapsed = now.elapsed().as_secs();

    let baselines = CachingTotalsBaselines {
        sqlite_none: 150,
        postgresql_none: 100,
        sqlite_truncate_all: 60,
        postgresql_truncate_all: 45,
        sqlite_truncate: 50,
        postgresql_truncate: 40,
        sqlite_trigger: 10,
        postgresql_trigger: 10,
        sqlite_memory: 10,
        postgresql_memory: 10,
    };

    match pool.get_caching_strategy() {
        CachingStrategy::None => match pool.kind() {
            DbKind::SQLite if elapsed > baselines.sqlite_none => {
                panic!("Took longer than {}s.", baselines.sqlite_none);
            }
            DbKind::PostgreSQL if elapsed > baselines.postgresql_none => {
                panic!("Took longer than {}s.", baselines.postgresql_none);
            }
            _ => (),
        },
        CachingStrategy::TruncateAll => match pool.kind() {
            DbKind::SQLite if elapsed > baselines.sqlite_truncate_all => {
                panic!("Took longer than {}s.", baselines.sqlite_truncate);
            }
            DbKind::PostgreSQL if elapsed > baselines.postgresql_truncate_all => {
                panic!("Took longer than {}s.", baselines.postgresql_truncate);
            }
            _ => (),
        },
        CachingStrategy::Truncate => match pool.kind() {
            DbKind::SQLite if elapsed > baselines.sqlite_truncate => {
                panic!("Took longer than {}s.", baselines.sqlite_truncate);
            }
            DbKind::PostgreSQL if elapsed > baselines.postgresql_truncate => {
                panic!("Took longer than {}s.", baselines.postgresql_truncate);
            }
            _ => (),
        },
        CachingStrategy::Trigger => match pool.kind() {
            DbKind::SQLite if elapsed > baselines.sqlite_trigger => {
                panic!("Took longer than {}s.", baselines.sqlite_trigger);
            }
            DbKind::PostgreSQL if elapsed > baselines.postgresql_trigger => {
                panic!("Took longer than {}s.", baselines.postgresql_trigger);
            }
            _ => (),
        },
        CachingStrategy::Memory(_) => match pool.kind() {
            DbKind::SQLite if elapsed > baselines.sqlite_memory => {
                panic!("Took longer than {}s.", baselines.sqlite_memory);
            }
            DbKind::PostgreSQL if elapsed > baselines.postgresql_memory => {
                panic!("Took longer than {}s.", baselines.postgresql_memory);
            }
            _ => (),
        },
    };

    // Output a blank line to make the overall output more readable:
    println!("");

    // Clean up:
    for table in &tables_to_choose_from {
        pool.drop_table(table).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    let opts = Opts::parse();
    match &opts.command {
        Subcommands::Caching { .. } => caching_performance(&opts).await,
    }
}
