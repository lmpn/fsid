use crate::application::port::outgoing::load_records::LoadRecords;

mod csv_handler;
pub fn create_load_records() -> Box<dyn LoadRecords> {
    Box::new(csv_handler::CSVHandler::new())
}
