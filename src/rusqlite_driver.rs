use anyhow::Result;
use async_trait::async_trait;
use deadpool_sqlite::{Config, Pool, Runtime};
use rlt::{BenchSuite, IterInfo, IterReport, Status, cli::BenchCli};
use std::time::Instant;

#[derive(Clone)]
pub(crate) struct RusqliteDriver {
    name: &'static str,
    tests_run: usize,
}

impl RusqliteDriver {
    pub async fn test(bench: &BenchCli) {
        let rusqlite_driver = RusqliteDriver {
            name: "rusqlite_driver",
            tests_run: 0,
        };
        rlt::cli::run(bench.clone(), rusqlite_driver).await.unwrap();
    }
}

#[async_trait]
impl BenchSuite for RusqliteDriver {
    type WorkerState = Pool;

    /// Initialize the state for a worker
    async fn state(&self, _worker_id: u32) -> Result<Self::WorkerState> {
        eprintln!("Connecting to the sqlite database using {}.", self.name);
        let cfg = Config::new(":memory:");
        let pool = cfg.create_pool(Runtime::Tokio1)?;
        Ok(pool)
    }

    /// Setup procedure before each worker starts.
    async fn setup(&mut self, pool: &mut Self::WorkerState, _worker_id: u32) -> Result<()> {
        eprintln!("Preparing the database.");
        let conn = pool.get().await?;
        conn.interact(move |conn| {
            let mut sql = String::from(
                "DROP TABLE IF EXISTS rltbl_driver; \
                 CREATE TABLE rltbl_driver (foo INT, bar TEXT); \
                 CREATE VIEW rltbl_driver_view AS SELECT * FROM rltbl_driver;",
            );

            // Add a few tens of thousands of values to the table:
            let mut values = vec![];
            for i in 0..5 {
                for j in 0..30000 {
                    values.push(format!("({i}, '{j}')"));
                }
            }
            let values = values.join(", ");
            sql.push_str(&format!(
                "INSERT INTO rltbl_driver (foo, bar) VALUES {}",
                values
            ));
            conn.execute_batch(&sql).unwrap();
        })
        .await
        .unwrap();
        Ok(())
    }

    /// Run the test.
    async fn bench(&mut self, pool: &mut Self::WorkerState, _: &IterInfo) -> Result<IterReport> {
        let start = Instant::now();

        let conn = pool.get().await?;
        conn.interact(move |conn| {
            let sql = "SELECT foo, COUNT(bar) \
                       FROM rltbl_driver_view \
                       WHERE foo > ?1 \
                       GROUP BY foo \
                       HAVING COUNT(bar) > ?2 \
                       ORDER BY foo";
            let mut stmt = conn.prepare(&sql).unwrap();
            let mut rows = stmt.query([&0, &20]).unwrap();
            while let Some(_row) = rows.next().unwrap() {
                // Do nothing. The point of this loop is just to consume the iterator.
            }

            if rand::random() && rand::random() {
                let sql = "INSERT INTO rltbl_driver (foo, bar) VALUES (?1, ?2)";
                let mut stmt = conn.prepare(&sql).unwrap();
                let _ = stmt.query([&1, &1]).unwrap();
            }
        })
        .await
        .unwrap();

        let duration = start.elapsed();
        self.tests_run += 1;

        Ok(IterReport {
            duration,
            status: Status::success(0),
            // Not used:
            items: 0,
            bytes: 0,
        })
    }

    // Teardown procedure after each worker finishes.
    async fn teardown(self, _pool: Self::WorkerState, _info: IterInfo) -> Result<()> {
        eprintln!(
            "Test is over after {} iterations. Tearing down.",
            self.tests_run
        );
        Ok(())
    }
}
