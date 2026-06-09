use anyhow::Result;
use async_trait::async_trait;
use rand::{
    SeedableRng as _,
    distr::{Distribution as _, Uniform},
    rngs::StdRng,
};
use rlt::{IterInfo, IterReport, StatelessBenchSuite, Status};
use rltbl_db::{any::AnyPool, core::DbQuery};
use tokio::time::Instant;

#[derive(Clone)]
pub(crate) struct CachingPerformance {
    pub(crate) seed: i64,
    pub(crate) pool: AnyPool,
    pub(crate) tables: Vec<String>,
    pub(crate) edit_rate: usize,
}

#[async_trait]
impl StatelessBenchSuite for CachingPerformance {
    async fn bench(&mut self, _: &IterInfo) -> Result<IterReport> {
        let start = Instant::now();
        self.perform_caching_detail().await;
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

impl CachingPerformance {
    pub fn random_between(min: usize, max: usize, seed: &mut i64) -> usize {
        let between = Uniform::try_from(min..max).unwrap();
        let mut rng = if *seed < 0 {
            StdRng::from_rng(&mut rand::rng())
        } else {
            *seed += 10;
            StdRng::seed_from_u64(*seed as u64)
        };
        between.sample(&mut rng)
    }

    fn random_table<'a>(&mut self) -> String {
        self.tables[Self::random_between(0, self.tables.len(), &mut self.seed)].to_string()
    }

    async fn perform_caching_detail(&mut self) {
        let select_table = self.random_table();
        self.pool
            .cache(
                &format!("SELECT foo, SUM(bar) FROM {select_table}_view GROUP BY foo ORDER BY foo"),
                (),
            )
            .await
            .unwrap();
        if self.edit_rate != 0 && Self::random_between(0, self.edit_rate, &mut self.seed) == 0 {
            let table_to_edit = self.random_table();
            self.pool
                .execute(
                    &format!("INSERT INTO {table_to_edit} (foo) VALUES (1), (1)"),
                    (),
                )
                .await
                .unwrap();
        }
    }
}
