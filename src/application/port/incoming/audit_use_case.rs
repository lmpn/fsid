use std::error::Error;

use crate::domain::grade::Grade;

#[derive(Default)]
pub struct AuditCommand {
    data: String,
    min_sources: usize,
    min_size: usize,
    max_distance: f64,
}

impl AuditCommand {
    pub fn new_with_defaults(data: String) -> Self {
        Self {
            data,
            min_sources: 2,
            min_size: 3,
            max_distance: 2.0,
        }
    }

    #[allow(dead_code)]
    pub fn new(data: String, min_sources: usize, min_size: usize, max_distance: f64) -> Self {
        Self {
            data,
            min_sources,
            min_size,
            max_distance,
        }
    }

    pub fn data(&self) -> &str {
        self.data.as_ref()
    }

    pub fn min_sources(&self) -> usize {
        self.min_sources
    }

    pub fn min_size(&self) -> usize {
        self.min_size
    }

    pub fn max_distance(&self) -> f64 {
        self.max_distance
    }
}

pub trait AuditUseCase {
    fn audit(&self, command: AuditCommand) -> Result<Vec<(String, Grade)>, Box<dyn Error>>;
}
