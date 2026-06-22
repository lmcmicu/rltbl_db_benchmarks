use clap::{Parser, Subcommand};
use rlt::cli::BenchCli;

mod rltbl_driver;
use rltbl_driver::RltblDriver;

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
    RltblDriver {
        #[clap(default_value = "rusqlite")]
        driver: String,
    },
}

#[tokio::main]
async fn main() {
    let opts = Opts::parse();
    match &opts.command {
        Subcommands::RltblDriver { driver } => RltblDriver::test(driver, &opts.bench).await,
    }
}
