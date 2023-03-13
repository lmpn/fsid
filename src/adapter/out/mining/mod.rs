use crate::application::port::outgoing::{
    get_all_distances::GetAllDistances, get_distance::GetDistance,
};

mod bold_miner;
#[allow(dead_code)]
pub fn create_get_bin_distance() -> Box<dyn GetDistance> {
    Box::new(bold_miner::BoldMiner::new())
}

#[allow(dead_code)]
pub fn create_get_all_bin_distance() -> Box<dyn GetAllDistances> {
    Box::new(bold_miner::BoldMiner::new())
}
