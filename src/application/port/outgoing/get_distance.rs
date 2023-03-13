use std::error::Error;

use crate::domain::distance::Distance;

pub trait GetDistance {
    fn get_distance(&self, bin: &str) -> Result<Distance, Box<dyn Error>>;
}
