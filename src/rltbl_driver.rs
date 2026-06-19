use anyhow::Result;
use async_trait::async_trait;
use rlt::{BenchSuite, IterInfo, IterReport, Status, cli::BenchCli};
use rltbl_db::{any::AnyPool, core::DbQuery};
use std::time::Instant;

#[derive(Clone)]
enum Driver {
    TokioPostgreSQL,
    Rusqlite,
    LibSQL,
}

#[derive(Clone)]
pub(crate) struct RltblDriver {
    driver: Driver,
}

impl RltblDriver {
    fn new(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "tokio-postgresql" => Self {
                driver: Driver::TokioPostgreSQL,
            },
            "rusqlite" => Self {
                driver: Driver::Rusqlite,
            },
            "libsql" => Self {
                driver: Driver::LibSQL,
            },
            _ => panic!("Invalid driver: '{name}'"),
        }
    }
}

impl RltblDriver {
    pub async fn test_rltbl(name: &str, bench: &BenchCli) {
        let rltbl_driver = RltblDriver::new(name);
        rlt::cli::run(bench.clone(), rltbl_driver).await.unwrap();
    }
}

#[async_trait]
impl BenchSuite for RltblDriver {
    type WorkerState = AnyPool;

    // The comment below is from the source code for the trait in rlt, but I think what it
    // actually does is initialize the state for all of the workers.
    // That said, maybe what needs to be done to get a per-worker state is to somehow
    // use the worker_id.
    // Initialize the state for a worker
    async fn state(&self, _worker_id: u32) -> Result<Self::WorkerState> {
        let pool = {
            let url = match self.driver {
                Driver::Rusqlite | Driver::LibSQL => ":memory:",
                Driver::TokioPostgreSQL => "postgresql:///rltbl_db",
            };
            AnyPool::connect(url).await.unwrap()
        };
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
        state
            .execute("CREATE TABLE rltbl_driver (foo INT, bar INT)", ())
            .await
            .unwrap();
        Ok(())
    }

    // The comment below is from the source code for the trait in rlt, but I think what it
    // actually does is to run the teardown procedure for all of the workers, i.e., after they
    // are all done.
    // That said, maybe what needs to be done to get a per-worker teardown is to somehow
    // use the worker_id.
    // Teardown procedure after each worker finishes.
    async fn teardown(self, state: Self::WorkerState, _info: IterInfo) -> Result<()> {
        state.drop_table("rltbl_driver").await.unwrap();
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
        state.execute(&sql, ()).await.unwrap();
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
