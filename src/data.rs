use serde::{Deserialize, Serialize};

pub trait FromData {
    fn from_data(data: &[Data]) -> Self;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Data {
    pub name: String,
    pub part: String,
    pub operator: String,
    pub replicate: usize,
    pub measured: f64,
    pub nominal: f64,
}

impl Default for Data {
    fn default() -> Self {
        Self::new()
    }
}

impl Data {
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
