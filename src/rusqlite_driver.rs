use anyhow::Result;
use async_trait::async_trait;
use deadpool_sqlite::{Config, Pool, Runtime};
use rlt::{BenchSuite, IterInfo, IterReport, Status, cli::BenchCli};
use std::time::Instant;

#[derive(Clone)]
pub(crate) struct RusqliteDriver {
    pool: Pool,
}

impl RusqliteDriver {
    async fn new() -> Self {
        let cfg = Config::new(":memory:");
        let pool = cfg.create_pool(Runtime::Tokio1).unwrap();

        let conn = pool.get().await.unwrap();
        conn.interact(move |conn| {
            let mut stmt = conn.prepare("DROP TABLE IF EXISTS rltbl_driver").unwrap();
            let _ = stmt.query([]).unwrap();

            let mut stmt = conn
                .prepare("CREATE TABLE rltbl_driver (foo INT, bar INT)")
                .unwrap();
            let _ = stmt.query([]).unwrap();

            let mut stmt = conn
                .prepare("CREATE VIEW rltbl_driver_view AS SELECT * FROM rltbl_driver")
                .unwrap();
            let _ = stmt.query([]).unwrap();

            // Add a few tens of thousands of values to the table:
            let mut values = vec![];
            for i in 0..5 {
                for j in 0..30000 {
                    values.push(format!("({i}, {j})"));
                }
            }
            let values = values.join(", ");
            let mut stmt = conn
                .prepare(&format!(
                    "INSERT INTO rltbl_driver (foo, bar) VALUES {}",
                    values
                ))
                .unwrap();
            let _ = stmt.query([]).unwrap();
        })
        .await
        .unwrap();

        Self { pool }
    }

    pub async fn test(bench: &BenchCli) {
        let driver = RusqliteDriver::new().await;
        rlt::cli::run(bench.clone(), driver).await.unwrap();
    }
}

#[async_trait]
impl BenchSuite for RusqliteDriver {
    type WorkerState = String;

    // The comment below is from the source code for the trait in rlt, but I think what it
    // actually does is initialize the state for all of the workers.
    // That said, maybe what needs to be done to get a per-worker state is to somehow
    // use the worker_id.
    // Initialize the state for a worker
    async fn state(&self, _worker_id: u32) -> Result<Self::WorkerState> {
        Ok("Good".to_string())
    }

    // The comment below is from the source code for the trait in rlt, but I think what it
    // actually does is to run the setup procedure for all of the workers (as judged by the
    // number of rows observed in each of the four tables once the test is running), i.e.,
    // before any of them run.
    // That said, maybe what needs to be done to get a per-worker setup is to somehow
    // use the worker_id.
    // Setup procedure before each worker starts.
    async fn setup(&mut self, _: &mut Self::WorkerState, _worker_id: u32) -> Result<()> {
        Ok(())
    }

    // The comment below is from the source code for the trait in rlt, but I think what it
    // actually does is to run the teardown procedure for all of the workers, i.e., after they
    // are all done.
    // That said, maybe what needs to be done to get a per-worker teardown is to somehow
    // use the worker_id.
    // Teardown procedure after each worker finishes.
    async fn teardown(self, _: Self::WorkerState, _info: IterInfo) -> Result<()> {
        Ok(())
    }

    async fn bench(&mut self, _: &mut Self::WorkerState, _: &IterInfo) -> Result<IterReport> {
        let start = Instant::now();

        let conn = self.pool.get().await.unwrap();
        conn.interact(move |conn| {
            let mut stmt = conn
                .prepare("SELECT foo, SUM(bar) FROM rltbl_driver_view GROUP BY foo ORDER BY foo")
                .unwrap();
            let _ = stmt.query([]).unwrap();

            if rand::random() && rand::random() {
                let mut stmt = conn
                    .prepare("INSERT INTO rltbl_driver (foo) VALUES (1), (1)")
                    .unwrap();
                let _ = stmt.query([]).unwrap();
            }
        })
        .await
        .unwrap();

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
