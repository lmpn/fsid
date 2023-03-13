use std::collections::HashMap;

use regex::Regex;

use crate::application::port::outgoing::{
    get_all_distances::GetAllDistances, get_distance::GetDistance,
};
use crate::domain::distance::Distance;

pub(crate) struct BoldMiner {
    re: Regex,
    url: String,
}
impl BoldMiner {
    pub fn new() -> Self {
        let re = Regex::new(
            r"Distance to Nearest Neighbor:</th>\s+<td>(?P<distance>\d+.\d+)%.*</td>(?s).*Nearest BIN URI:</th>\s+<td>(?P<bin>\w+:\w+)</td>",
        );
        Self {
            re: re.unwrap(),
            url: "http://v4.boldsystems.org/index.php/Public_BarcodeCluster?clusteruri="
                .to_string(),
        }
    }
}

impl GetDistance for BoldMiner {
    fn get_distance(&self, bin: &str) -> Result<Distance, Box<dyn std::error::Error>> {
        let formatted_url = format!("{}{}", self.url, bin);
        let text = reqwest::blocking::get(formatted_url)?.text()?;
        let captures = self.re.captures(&text).unwrap();
        let bin = Distance::new(
            bin.to_string(),
            captures.name("bin").unwrap().as_str().to_string(),
            captures.name("distance").unwrap().as_str().parse().unwrap(),
        );
        Ok(bin)
    }
}

impl GetAllDistances for BoldMiner {
    fn get_all_distances(&self, bins: &[String]) -> HashMap<String, Distance> {
        let mut distance = HashMap::new();
        let mut evicted = vec![];
        for bin_uri in bins {
            if distance.contains_key(bin_uri) || evicted.contains(bin_uri) {
                continue;
            }
            match self.get_distance(bin_uri) {
                Ok(bin) => {
                    let inv = bin.inverse();
                    distance.insert(inv.bin_uri_a().to_string(), inv);
                    distance.insert(bin.bin_uri_a().to_string(), bin);
                }
                Err(err) => {
                    evicted.push((*bin_uri).to_owned());
                    println!("{err}");
                }
            }
        }
        distance
    }
}
