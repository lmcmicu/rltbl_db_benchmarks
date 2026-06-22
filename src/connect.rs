use anyhow::Result;
use async_trait::async_trait;
use rlt::{BenchSuite, IterInfo, IterReport, Status, cli::BenchCli};
use rltbl_db::any::AnyPool;
use std::time::Instant;

#[derive(Clone)]
pub(crate) struct Connect {
    url: String,
}

impl Connect {
    pub async fn test(url: &str, bench: &BenchCli) {
        rlt::cli::run(
            bench.clone(),
            Connect {
                url: url.to_string(),
            },
        )
        .await
        .unwrap();
    }
}

#[async_trait]
impl BenchSuite for Connect {
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
        let url = match self.url.to_lowercase().as_str() {
            "tokio" | "tokio-postgres" | "tokio-postgresql" => "postgresql:///rltbl_db",
            "rusqlite" | "libsql" => ":memory:",
            _ => panic!("Invalid URL: '{}'", self.url),
        };

        let start = Instant::now();
        let _ = AnyPool::connect(url).await.unwrap();
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
