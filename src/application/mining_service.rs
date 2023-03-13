use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use futures::future::join_all;

use crate::domain::{distance::Distance, record::Record};

use super::port::{
    incoming::mining_use_case::{MiningCommand, MiningUseCase},
    outgoing::{get_distance::GetDistance, load_records::LoadRecords},
};

pub struct MiningService {
    get_distance_port: Box<dyn GetDistance>,
    load_records_port: Box<dyn LoadRecords>,
}

impl MiningService {
    pub fn new(
        load_records_port: Box<dyn LoadRecords>,
        get_distance_port: Box<dyn GetDistance>,
    ) -> Self {
        Self {
            load_records_port,
            get_distance_port,
        }
    }

    async fn mine_bin_uri(&self, bin_uri: &str) -> Distance {
        self.get_distance_port
            .get_distance(bin_uri)
            .unwrap_or(Distance::new_empty(bin_uri.to_string()))
    }

    fn mine_algo(&self, records: &HashSet<Record>) -> HashMap<String, Distance> {
        let futures = records.into_iter().map(|v| self.mine_bin_uri(v.bin_uri()));
        let j = join_all(futures);

        let v = futures::executor::block_on(j);
        v.into_iter()
            .map(|v| (v.bin_uri_a().to_string(), v))
            .collect::<HashMap<String, Distance>>()
    }
}

impl MiningUseCase for MiningService {
    fn mine(&self, command: MiningCommand) -> Result<HashMap<String, Distance>, Box<dyn Error>> {
        let records = self
            .load_records_port
            .load(command.data(), b'\t')?
            .into_iter()
            .filter(|r| !r.bin_uri().is_empty())
            .collect::<HashSet<Record>>();
        Ok(self.mine_algo(&records))
    }
}
