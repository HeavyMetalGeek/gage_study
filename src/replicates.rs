use crate::data::{Data, FromData};
use std::collections::HashMap;

impl FromData for Vec<Replicate> {
    fn from_data(data: &[Data]) -> Self {
        let mut replicates: HashMap<(String, String), Replicate> = HashMap::new();
        data.iter().for_each(|d| {
            let replicate = replicates
                .entry((d.part.clone(), d.operator.clone()))
                .or_insert_with(|| Replicate::new(&d.part, &d.operator));
            replicate.values.push(d.measured);
        });
        replicates.into_values().collect()
    }
}

#[derive(Debug, Clone)]
pub struct Replicate {
    pub part: String,
    pub operator: String,
    pub values: Vec<f64>,
}

impl Default for Replicate {
    fn default() -> Self {
        Self::new("", "")
    }
}

impl Replicate {
    pub fn new(part: &str, operator: &str) -> Self {
        Self {
            part: part.to_owned(),
            operator: operator.to_owned(),
            values: Vec::new(),
        }
    }
    pub fn mean(&self) -> f64 {
        let sum: f64 = self.values.iter().sum();
        let count: f64 = self.values.len() as f64;
        sum / count
    }
    pub fn sqdiff(&self) -> f64 {
        let mean = self.mean();
        self.values.iter().map(|v| (v - mean).powi(2)).sum()
    }
}
