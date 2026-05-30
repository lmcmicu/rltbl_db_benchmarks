use anyhow::Result;
use async_trait::async_trait;
use rlt::{IterInfo, IterReport, StatelessBenchSuite, Status};
use rltbl_db::{any::AnyPool, core::DbQuery, db_kind::DbKind, db_row, db_value::DbRow};
use std::{thread, time::Duration};
use tokio::time::Instant;

#[derive(Clone)]
pub(crate) struct BenchmarkExample1 {
    pub(crate) kind: DbKind,
}

#[async_trait]
impl StatelessBenchSuite for BenchmarkExample1 {
    async fn bench(&mut self, _: &IterInfo) -> Result<IterReport> {
        let t = Instant::now();

        let url = match self.kind {
            DbKind::SQLite => ":memory:",
            DbKind::PostgreSQL => "postgresql:///rltbl_db",
        };
        let pool = AnyPool::connect(url).await.unwrap();
        let cascade = match self.kind {
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

        pool.query("SELECT 1 FROM test1", ()).await.unwrap();

        thread::sleep(Duration::from_millis(100));

        let duration = t.elapsed();

        let report = IterReport {
            duration,
            status: Status::success(0),
            bytes: 0,
            items: 0,
        };
        Ok(report)
    }
}
