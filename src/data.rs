use serde::{Deserialize, Serialize};

/// Provides a conversion mechanism from [Data](crate::data::Data) structs
pub trait FromData {
    fn from_data(data: &[Data]) -> Self;
}

/// An individual data point.  This is the primary strucure used for deserialization.
#[derive(Clone, Serialize, Deserialize)]
pub struct Data {
    pub name: String,
    pub part: String,
    pub operator: String,
    pub replicate: usize,
    pub measured: f64,
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
}
