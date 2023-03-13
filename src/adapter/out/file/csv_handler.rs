use std::error::Error;

use crate::application::port::outgoing::load_records::LoadRecords;
use crate::domain::record::Record;
pub(crate) struct CSVHandler {}
impl CSVHandler {
    pub fn new() -> Self {
        Self {}
    }
}

impl LoadRecords for CSVHandler {
    fn load(&self, path: &str, delimiter: u8) -> Result<Vec<Record>, Box<dyn Error>> {
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(delimiter)
            .from_path(path)?;
        let mut records = vec![];
        for result in rdr.deserialize() {
            let record: Record = result?;
            records.push(record);
        }
        Ok(records)
    }
}
