use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Record {
    species_name: String,
    institution_storing: String,
    bin_uri: String,
}
impl Record {
    pub(crate) fn is_valid(&self) -> bool {
        !(self.species_name.is_empty()
            || self.bin_uri.is_empty()
            || self.institution_storing.is_empty())
    }
    pub(crate) fn species_name(&self) -> &String {
        &self.species_name
    }
    pub(crate) fn institution_storing(&self) -> &String {
        &self.institution_storing
    }
    pub(crate) fn bin_uri(&self) -> &str {
        &self.bin_uri
    }
}
