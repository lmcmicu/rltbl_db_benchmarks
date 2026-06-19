use clap::{Parser, Subcommand};
use rlt::cli::BenchCli;

mod caching_performance;
use caching_performance::CachingPerformance;

mod rltbl_driver;
use rltbl_driver::RltblDriver;

#[derive(Parser, Clone)]
struct Opts {
    #[clap(long, default_value = "-1")]
    seed: i64,

    #[clap(long)]
    totals_file: String,

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
    Rltbl {
        #[clap(default_value = "rusqlite")]
        driver: String,
    },
    Tokio { },
    Rusqlite { },
    Libsql { },
}

#[tokio::main]
async fn main() {
    let opts = Opts::parse();
    match &opts.command {
        Subcommands::Caching {
            kind,
            strategy,
            edit_rate,
        } => {
            CachingPerformance::perform_caching(
                &kind,
                &opts.bench,
                strategy,
                *edit_rate,
                &opts.totals_file,
                opts.seed,
            )
            .await
        }
        Subcommands::Rltbl { driver } => RltblDriver::test_rltbl(driver, &opts.bench).await,
        Subcommands::Tokio { } => todo!(),
        Subcommands::Rusqlite { } => todo!(),
        Subcommands::Libsql { } => todo!(),
    }
}
