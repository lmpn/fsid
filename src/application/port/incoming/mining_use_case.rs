use std::{collections::HashMap, error::Error};

use crate::domain::distance::Distance;

#[derive(Default)]
pub struct MiningCommand {
    data: String,
}

impl MiningCommand {
    #[allow(dead_code)]
    pub fn new(data: String) -> Self {
        Self { data }
    }

    pub fn data(&self) -> &str {
        self.data.as_ref()
    }
}

pub trait MiningUseCase {
    fn mine(&self, command: MiningCommand) -> Result<HashMap<String, Distance>, Box<dyn Error>>;
}
