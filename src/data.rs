use serde::{Deserialize, Serialize};

/// Provides a conversion mechanism from [Data](crate::data::Data) structs
pub trait FromData {
    fn from_data(data: &[Data]) -> Self;
}

/// An individual data point.  This is the primary strucure used for deserialization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    #[serde(default)]
    pub name: String,
    pub part: String,
    pub operator: String,
    pub replicate: usize,
    pub measured: f64,
    #[serde(default)]
    pub nominal: f64,
}

/// Makes the default Data
impl Default for Data {
    fn default() -> Self {
        Self::new()
    }
}

impl Data {
    /// Makes a new Data
    pub fn new() -> Self {
        Self {
            name: String::new(),
            part: String::new(),
            operator: String::new(),
            replicate: 0,
            measured: 0.0,
            nominal: 0.0,
        }
    }

    pub fn from_raw(raw: &[u8], ext: &str) -> Vec<Self> {
        if ext == "csv" {
            Data::from_raw_csv(raw)
        } else if ext == "json" {
            Data::from_raw_json(raw)
        } else {
            Vec::new()
        }
    }

    pub fn from_raw_csv(raw: &[u8]) -> Vec<Self> {
        csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(raw)
            .deserialize()
            .filter_map(Result::ok)
            .collect()
    }

    pub fn from_raw_json(raw: &[u8]) -> Vec<Self> {
        match serde_json::from_slice(raw) {
            Ok(d) => d,
            Err(_) => Vec::new(),
        }
    }
}
