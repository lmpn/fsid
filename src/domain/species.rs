use std::collections::HashSet;

use super::{grade::Grade, record::Record};

#[derive(Debug)]
pub struct Species {
    species_name: String,
    institution_storing: HashSet<String>,
    bin_uri: HashSet<String>,
    grade: Grade,
    count: usize,
}

impl Species {
    pub(crate) fn new(
        species_name: String,
        institution_storing: HashSet<String>,
        bin_uri: HashSet<String>,
        grade: Grade,
        count: usize,
    ) -> Self {
        Self {
            species_name,
            institution_storing,
            bin_uri,
            grade,
            count,
        }
    }
    pub(crate) fn add(&mut self, r: &Record) {
        self.institution_storing
            .insert(r.institution_storing().to_string());
        self.bin_uri.insert(r.bin_uri().to_string());
        self.count += 1;
    }

    #[allow(dead_code)]
    pub fn species_name(&self) -> &str {
        self.species_name.as_ref()
    }

    pub fn institution_storing(&self) -> &HashSet<String> {
        &self.institution_storing
    }

    #[allow(dead_code)]
    pub fn grade(&self) -> &Grade {
        &self.grade
    }

    pub fn bin_uri(&self) -> &HashSet<String> {
        &self.bin_uri
    }

    pub fn count(&self) -> usize {
        self.count
    }

    #[allow(dead_code)]
    pub fn set_grade(&mut self, grade: Grade) {
        self.grade = grade;
    }
}
