#[derive(Debug)]
pub struct Distance {
    bin_uri_a: String,
    bin_uri_b: String,
    distance: f64,
}

impl Distance {
    pub fn new_empty(bin_uri_a: String) -> Self {
        Self {
            bin_uri_a,
            bin_uri_b: "".to_string(),
            distance: f64::MAX,
        }
    }
    pub fn new(bin_uri_a: String, bin_uri_b: String, distance: f64) -> Self {
        Self {
            bin_uri_a,
            bin_uri_b,
            distance,
        }
    }

    pub(crate) fn inverse(&self) -> Self {
        Self {
            bin_uri_a: self.bin_uri_b.clone(),
            bin_uri_b: self.bin_uri_a.clone(),
            distance: self.distance,
        }
    }

    pub fn bin_uri_b(&self) -> &str {
        self.bin_uri_b.as_ref()
    }

    pub fn bin_uri_a(&self) -> &str {
        self.bin_uri_a.as_ref()
    }

    pub fn distance(&self) -> f64 {
        self.distance
    }
}
