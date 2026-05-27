use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use rlt::{IterInfo, IterReport, StatelessBenchSuite, Status, cli::BenchCli};
use rltbl_db::{any::AnyPool, core::DbQuery, db_kind::DbKind, db_row, db_value::DbRow};
use std::{thread, time::Duration};
use tokio::time::Instant;

#[derive(Clone)]
struct Bench1 {
    kind: DbKind,
}

#[derive(Clone)]
struct Bench2 {
    kind: DbKind,
}

#[async_trait]
impl StatelessBenchSuite for Bench1 {
    async fn bench(&mut self, _: &IterInfo) -> Result<IterReport> {
        let t = Instant::now();

        let url = match self.kind {
            DbKind::SQLite => ":memory:",
            DbKind::PostgreSQL => "postgresql:///rltbl_db",
        };
        let pool = AnyPool::connect(url).await.unwrap();
        let cascade = match pool.kind() {
            DbKind::SQLite => "",
            DbKind::PostgreSQL => " CASCADE",
        };
        pool.execute_batch(&format!(
            "DROP TABLE IF EXISTS test1{cascade};\
             CREATE TABLE test1 ( value TEXT )",
        ))
        .await
        .unwrap();

        pool.insert(
            "test1",
            &["value"],
            &[&db_row! {
                "value" => "foo",
            }],
        )
        .await
        .unwrap();

        let _: Vec<DbRow> = pool.query("SELECT 1 FROM test1", ()).await.unwrap();
        let _: Vec<DbRow> = pool.query("SELECT 1 FROM test1", ()).await.unwrap();

        thread::sleep(Duration::from_millis(100));

        let duration = t.elapsed();

        let report = IterReport {
            duration,
            status: Status::success(0),
            bytes: 42, // TODO: Add the actual value of: bytes processed in current iteration
            items: 5,  // TODO: Add the actual value of: items processed in current iteration
        };
        Ok(report)
    }
}

#[async_trait]
impl StatelessBenchSuite for Bench2 {
    async fn bench(&mut self, _: &IterInfo) -> Result<IterReport> {
        let t = Instant::now();

        let url = match self.kind {
            DbKind::SQLite => ":memory:",
            DbKind::PostgreSQL => "postgresql:///rltbl_db",
        };
        let pool = AnyPool::connect(url).await.unwrap();
        let cascade = match pool.kind() {
            DbKind::SQLite => "",
            DbKind::PostgreSQL => " CASCADE",
        };
        pool.execute_batch(&format!(
            "DROP TABLE IF EXISTS test2{cascade};\
             CREATE TABLE test2 ( value TEXT )",
        ))
        .await
        .unwrap();

        pool.insert(
            "test2",
            &["value"],
            &[&db_row! {
                "value" => "foo",
            }],
        )
        .await
        .unwrap();

        let _: Vec<DbRow> = pool.query("SELECT 1 FROM test2", ()).await.unwrap();
        let _: Vec<DbRow> = pool.query("SELECT 1 FROM test2", ()).await.unwrap();

        thread::sleep(Duration::from_millis(100));

        let duration = t.elapsed();

        let report = IterReport {
            duration,
            status: Status::success(0),
            bytes: 42, // TODO: Add the actual value of: bytes processed in current iteration
            items: 5,  // TODO: Add the actual value of: items processed in current iteration
        };
        Ok(report)
    }
}

#[tokio::main]
async fn main() {
    for kind in [DbKind::SQLite, DbKind::PostgreSQL] {
        rlt::cli::run(BenchCli::parse(), Bench1 { kind })
            .await
            .unwrap();

        rlt::cli::run(BenchCli::parse(), Bench2 { kind })
            .await
            .unwrap();
    }
}
