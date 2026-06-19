use anyhow::Result;
use async_trait::async_trait;
use deadpool_postgres::{Config, Pool, Runtime, tokio_postgres::NoTls};
use rlt::{BenchSuite, IterInfo, IterReport, Status, cli::BenchCli};
use std::time::Instant;

#[derive(Clone)]
pub(crate) struct TokioDriver;

impl TokioDriver {
    pub async fn test_tokio(bench: &BenchCli) {
        rlt::cli::run(bench.clone(), TokioDriver {}).await.unwrap();
    }
}

#[async_trait]
impl BenchSuite for TokioDriver {
    type WorkerState = Pool;

    // The comment below is from the source code for the trait in rlt, but I think what it
    // actually does is initialize the state for all of the workers.
    // That said, maybe what needs to be done to get a per-worker state is to somehow
    // use the worker_id.
    // Initialize the state for a worker
    async fn state(&self, _worker_id: u32) -> Result<Self::WorkerState> {
        let mut cfg = Config::new();
        let db_name = "rltbl_db";
        cfg.dbname = Some(db_name.to_string());
        let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
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
        let client = state.get().await.unwrap();
        let stmt = client
            .prepare_cached("CREATE TABLE rltbl_driver (foo INT, bar INT)")
            .await
            .unwrap();
        let _ = client.query(&stmt, &[]).await.unwrap();

        Ok(())
    }

    // The comment below is from the source code for the trait in rlt, but I think what it
    // actually does is to run the teardown procedure for all of the workers, i.e., after they
    // are all done.
    // That said, maybe what needs to be done to get a per-worker teardown is to somehow
    // use the worker_id.
    // Teardown procedure after each worker finishes.
    async fn teardown(self, state: Self::WorkerState, _info: IterInfo) -> Result<()> {
        let client = state.get().await.unwrap();
        let stmt = client
            .prepare_cached("DROP TABLE rltbl_driver")
            .await
            .unwrap();
        let _ = client.query(&stmt, &[]).await.unwrap();
        Ok(())
    }

    async fn bench(&mut self, state: &mut Self::WorkerState, _: &IterInfo) -> Result<IterReport> {
        let mut values = vec![];
        for i in 0..5 {
            for j in 0..1000 {
                values.push(format!("({i}, {j})"));
            }
        }
        let values = values.join(", ");
        let sql = format!("INSERT INTO rltbl_driver (foo, bar) VALUES {values}");

        let start = Instant::now();
        let client = state.get().await.unwrap();
        let stmt = client.prepare_cached(&sql).await.unwrap();
        let _ = client.query(&stmt, &[]).await.unwrap();

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
