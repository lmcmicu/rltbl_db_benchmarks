use clap::Parser;
use rlt::cli::BenchCli;
use rltbl_db::{
    any::AnyPool,
    core::{CachingStrategy, DbQuery},
    db_kind::DbKind,
};
use std::str::FromStr;

mod caching_performance;
use caching_performance::CachingPerformance;

#[derive(Parser, Clone)]
struct Opts {
    #[clap(default_value = "sqlite")]
    kind: String,

    #[clap(default_value = "none")]
    strategy: String,

    #[clap(long, default_value = "25")]
    edit_rate: usize,

    #[command(flatten)]
    bench: BenchCli,
}

/// TODO: Add docstring.
async fn caching_performance(opts: &Opts) {
    let mut pool = {
        let kind = DbKind::from_str(&opts.kind.as_str()).expect("Error reading database kind");
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
            for j in 0..CachingPerformance::random_between(34000, 35000, &mut -1) {
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
    pool.set_caching_strategy(&CachingStrategy::from_str(&opts.strategy).unwrap());

    let this_test = "Caching Performance Test -";
    println!(
        "{this_test} Starting test with db kind '{}' and strategy '{}'.",
        pool.kind(),
        pool.get_caching_strategy()
    );

    rlt::cli::run(
        opts.bench.clone(),
        CachingPerformance {
            pool: pool.clone(),
            tables: tables_to_choose_from
                .clone()
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>(),
            edit_rate: opts.edit_rate,
        },
    )
    .await
    .unwrap();

    // Output a blank line to make the overall output more readable:
    println!("");

    // Clean up:
    for table in &tables_to_choose_from {
        pool.drop_table(table).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    // TODO: Make the tests multi-threaded.

    let opts = Opts::parse();

    // TODO: Currently there is only one benchmark test. Eventually, when we add more benchmarks,
    // we should specify them as subcommands using command-line arguments in the definition
    // of Opts above.
    caching_performance(&opts).await;
}
