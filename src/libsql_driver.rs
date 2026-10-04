use anyhow::Result;
use async_trait::async_trait;
use deadpool_libsql::{self, Manager, Pool, libsql::Builder};
use rlt::{BenchSuite, IterInfo, IterReport, Status, cli::BenchCli};
use std::time::Instant;

#[derive(Clone)]
pub(crate) struct LibsqlDriver {
    name: &'static str,
    tests_run: usize,
}

impl LibsqlDriver {
    pub async fn test(bench: &BenchCli) {
        let libsql_driver = LibsqlDriver {
            name: "libsql_driver",
            tests_run: 0,
        };
        rlt::cli::run(bench.clone(), libsql_driver).await.unwrap();
    }
}

#[async_trait]
impl BenchSuite for LibsqlDriver {
    type WorkerState = Pool;

    /// Initialize the state for a worker
    async fn state(&self, _worker_id: u32) -> Result<Self::WorkerState> {
        eprintln!("Connecting to the sqlite database using {}.", self.name);
        let db = Builder::new_local(":memory:").build().await?;
        let manager = Manager::from_libsql_database(db);
        let pool = Pool::builder(manager).build()?;
        Ok(pool)
    }

    /// Setup procedure before each worker starts.
    async fn setup(&mut self, pool: &mut Self::WorkerState, _worker_id: u32) -> Result<()> {
        eprintln!("Preparing the database.");
        let conn = pool.get().await?;
        let _ = conn.query("DROP TABLE IF EXISTS rltbl_driver", ()).await?;
        let _ = conn
            .query("CREATE TABLE rltbl_driver (foo INT, bar TEXT)", ())
            .await?;
        let _ = conn
            .query(
                "CREATE VIEW rltbl_driver_view AS SELECT * FROM rltbl_driver",
                (),
            )
            .await?;

        // Add a few tens of thousands of values to the table:
        let mut values = vec![];
        for i in 0..5 {
            for j in 0..30000 {
                values.push(format!("({i}, '{j}')"));
            }
        }
        let values = values.join(", ");
        let stmt = format!("INSERT INTO rltbl_driver (foo, bar) VALUES {values}");
        let _ = conn.query(&stmt, ()).await?;
        Ok(())
    }

    /// Run the test.
    async fn bench(&mut self, pool: &mut Self::WorkerState, _: &IterInfo) -> Result<IterReport> {
        let start = Instant::now();

        let conn = pool.get().await?;
        let sql = "SELECT foo, COUNT(bar) \
                   FROM rltbl_driver_view \
                   WHERE foo > ?1 \
                   GROUP BY foo \
                   HAVING COUNT(bar) > ?2 \
                   ORDER BY foo";
        let mut rows = conn.query(sql, [0, 20]).await?;
        while let Some(_row) = rows.next().await? {
            // Do nothing. The point of this loop is just to consume the iterator.
        }

        if rand::random() && rand::random() {
            let sql = "INSERT INTO rltbl_driver (foo, bar) VALUES (?1, ?2)";
            let _ = conn.query(sql, [1, 1]).await?;
        }

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
