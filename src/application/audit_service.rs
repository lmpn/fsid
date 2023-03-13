use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use petgraph::{algo::connected_components, prelude::UnGraphMap};

use crate::domain::{distance::Distance, grade::Grade, record::Record, species::Species};

use super::port::{
    incoming::audit_use_case::{AuditCommand, AuditUseCase},
    outgoing::{get_distance::GetDistance, load_records::LoadRecords},
};

pub struct AuditService {
    load_records_port: Box<dyn LoadRecords>,
    get_distance_port: Box<dyn GetDistance>,
}

impl AuditService {
    pub fn new(
        load_records_port: Box<dyn LoadRecords>,
        get_distance_port: Box<dyn GetDistance>,
    ) -> Self {
        Self {
            load_records_port,
            get_distance_port,
        }
    }
    fn is_shared_bin(&self, ds: &HashMap<String, Species>, bin: &String) -> bool {
        let mut count: u64 = 0;
        for entry in ds {
            if entry.1.bin_uri().iter().any(|bu| bu == bin) {
                count += 1;
                if count == 2 {
                    return false;
                }
            }
        }
        true
    }

    fn get_neighbours<'a>(
        &self,
        bs: &'a HashSet<String>,
        distances: &'a HashMap<String, Distance>,
    ) -> Vec<Distance> {
        let mut neighbours = Vec::with_capacity(bs.len());
        for cluster in bs {
            let i = distances.iter().find(|entry| entry.0 == cluster);
            if let Some(entry) = i {
                neighbours.push(Distance::new(
                    entry.1.bin_uri_a().to_owned(),
                    entry.1.bin_uri_b().to_owned(),
                    entry.1.distance(),
                ));
            } else {
                let bin = self
                    .get_distance_port
                    .get_distance(cluster)
                    .unwrap_or(Distance::new(cluster.to_owned(), "".to_string(), f64::MAX));
                let inv_bin = bin.inverse();
                neighbours.push(bin);
                neighbours.push(inv_bin);
            }
        }
        neighbours
    }

    fn check_connectivity(
        &self,
        species: &Species,
        data: &HashMap<String, Species>,
        distances: &mut HashMap<String, Distance>,
        max_distance: f64,
    ) -> Grade {
        let clusters = species.bin_uri();
        let mut ctr = 0;
        for pair in data {
            let current_clusters = pair.1.bin_uri();
            let is = current_clusters.intersection(clusters).next();
            if is.is_some() {
                ctr += 1;
                if ctr > 1 {
                    return Grade::E;
                }
            }
        }
        let neighbours = self.get_neighbours(clusters, distances);
        let graph = neighbours
            .iter()
            .filter(|b| b.distance() <= max_distance)
            .map(|b| (b.bin_uri_a(), b.bin_uri_b()))
            .collect::<UnGraphMap<_, ()>>();
        let cc = connected_components(&graph);
        if cc == 1 {
            return Grade::C;
        }
        Grade::E
    }

    fn group(&self, records: Vec<Record>) -> HashMap<String, Species> {
        let mut ds: HashMap<String, Species> = HashMap::new();
        for record in records.iter() {
            ds.entry(record.species_name().to_string())
                .and_modify(|s| s.add(record))
                .or_insert_with(|| {
                    let mut bu = HashSet::new();
                    bu.insert(record.bin_uri().to_string());
                    let mut is = HashSet::new();
                    is.insert(record.institution_storing().to_string());
                    Species::new(record.species_name().to_string(), is, bu, Grade::E, 1)
                });
        }
        ds
    }

    fn audit_algo(
        &self,
        ds: &HashMap<String, Species>,
        distances: &mut HashMap<String, Distance>,
        min_size: usize,
        min_sources: usize,
        max_distance: f64,
    ) -> Vec<(String, Grade)> {
        let mut grades = vec![];

        for entry in ds.iter() {
            let species = entry.1;
            let mut grade = Grade::D;

            if species.institution_storing().len() >= min_sources {
                grade = Grade::E;
                if species.bin_uri().len() == 1 {
                    let bin = species.bin_uri().iter().next();
                    if let Some(b) = bin {
                        if self.is_shared_bin(ds, b) {
                            grade = if species.count() >= min_size {
                                Grade::A
                            } else {
                                Grade::B
                            };
                        }
                    }
                } else {
                    grade = self.check_connectivity(species, ds, distances, max_distance);
                }
            }
            grades.push((entry.0.clone(), grade));
        }
        grades
    }
}

impl AuditUseCase for AuditService {
    fn audit(&self, command: AuditCommand) -> Result<Vec<(String, Grade)>, Box<dyn Error>> {
        let mut records = self.load_records_port.load(command.data(), b'\t')?;
        records = records.drain(0..).filter(|r| r.is_valid()).collect();
        let mut distances = HashMap::new();
        let dataset = self.group(records);
        let grading = self.audit_algo(
            &dataset,
            &mut distances,
            command.min_size(),
            command.min_sources(),
            command.max_distance(),
        );
        Ok(grading)
    }
}
