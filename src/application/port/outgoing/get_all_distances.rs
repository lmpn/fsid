use std::collections::HashMap;

use crate::domain::distance::Distance;

pub trait GetAllDistances {
    fn get_all_distances(&self, s: &[String]) -> HashMap<String, Distance>;
}
