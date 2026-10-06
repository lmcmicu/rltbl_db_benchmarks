//! Command line interface for rltbl_db regression tests.

use clap::{Parser, Subcommand};
use rlt::cli::BenchCli;

mod caching_performance;
use caching_performance::CachingPerformance;

mod rltbl_driver;
use rltbl_driver::RltblDriver;

mod tokio_postgres_driver;
use tokio_postgres_driver::TokioPostgresDriver;

mod rusqlite_driver;
use rusqlite_driver::RusqliteDriver;


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
    /// Tests caching performance.
    Caching {
        #[clap(default_value = "sqlite")]
        kind: String,

        #[clap(default_value = "none")]
        strategy: String,

        #[clap(long, default_value = "25")]
        edit_rate: usize,

        #[clap(long, default_value = "")]
        totals_file: String,
    },
    /// Tests query performance using rltbl_db's generic driver.
    RltblDriver {
        #[clap(default_value = "rusqlite")]
        driver: String,
    },
    /// Tests query performance using deadpool's tokio-postgres driver directly.
    TokioPostgresDriver { },
    /// Tests query performance using deadpool's rusqlite driver directly.
    RusqliteDriver { },
}

#[tokio::main]
async fn main() {
    let opts = Opts::parse();
    match &opts.command {
        Subcommands::Caching {
            kind,
            strategy,
            edit_rate,
            totals_file,
        } => {
            CachingPerformance::test(
                &kind,
                &opts.bench,
                strategy,
                *edit_rate,
                totals_file,
                opts.seed,
            )
            .await
        }
        Subcommands::RltblDriver { driver } => RltblDriver::test(driver, &opts.bench).await,
        Subcommands::TokioPostgresDriver { } => TokioPostgresDriver::test(&opts.bench).await,
        Subcommands::RusqliteDriver { } => RusqliteDriver::test(&opts.bench).await,
    };
}
