use self::{
    audit_service::AuditService,
    mining_service::MiningService,
    port::{
        incoming::{audit_use_case::AuditUseCase, mining_use_case::MiningUseCase},
        outgoing::{get_distance::GetDistance, load_records::LoadRecords},
    },
};
mod audit_service;
mod mining_service;
pub mod port;

pub fn create_audit_use_case(
    load_records_port: Box<dyn LoadRecords>,
    get_distance_port: Box<dyn GetDistance>,
) -> Box<dyn AuditUseCase> {
    Box::new(AuditService::new(load_records_port, get_distance_port))
}

pub fn create_mining_use_case(
    load_records_port: Box<dyn LoadRecords>,
    get_distance_port: Box<dyn GetDistance>,
) -> Box<dyn MiningUseCase> {
    Box::new(MiningService::new(load_records_port, get_distance_port))
}
