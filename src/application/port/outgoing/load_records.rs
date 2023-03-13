use std::error::Error;

use crate::domain::record::Record;

pub trait LoadRecords {
    fn load(&self, path: &str, delimiter: u8) -> Result<Vec<Record>, Box<dyn Error>>;
}
