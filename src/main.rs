use clap::Parser;
use rlt::cli::BenchCli;
use rltbl_db::db_kind::DbKind;

mod benchmark_example1;
use benchmark_example1::BenchmarkExample1;
mod benchmark_example2;
use benchmark_example2::BenchmarkExample2;

#[tokio::main]
async fn main() {
    for kind in [DbKind::SQLite, DbKind::PostgreSQL] {
        rlt::cli::run(BenchCli::parse(), BenchmarkExample1 { kind })
            .await
            .unwrap();

        rlt::cli::run(BenchCli::parse(), BenchmarkExample2 { kind })
            .await
            .unwrap();
    }
}
