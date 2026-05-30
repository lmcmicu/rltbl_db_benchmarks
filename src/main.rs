use clap::Parser;
use rlt::{
    cli::BenchCli,
};
use rltbl_db::db_kind::DbKind;
use std::str::FromStr;

mod benchmark_example1;
use benchmark_example1::BenchmarkExample1;
mod benchmark_example2;
use benchmark_example2::BenchmarkExample2;

#[derive(Parser, Clone)]
struct Opts {
    /// Base latency for simulated work.
    ///
    /// Lower values simulate better performance.
    /// Examples: 100us, 1ms, 500us
    //#[clap(long, default_value = "50ms")]
    kind: String,

    #[command(flatten)]
    bench: BenchCli,
}


#[tokio::main]
async fn main() {
    let opts = Opts::parse();
    let kind = DbKind::from_str(&opts.kind.as_str()).expect("Error reading database kind");

    rlt::cli::run(opts.bench.clone(), BenchmarkExample1 { kind })
            .await
            .unwrap();

    rlt::cli::run(opts.bench, BenchmarkExample2 { kind })
            .await
            .unwrap();
}
