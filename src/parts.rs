use crate::anova::Anova;
use crate::data::{Data, FromData};
use std::collections::HashMap;

impl FromData for Vec<Part> {
    fn from_data(data: &[Data]) -> Self {
        let mut parts: HashMap<String, Part> = HashMap::new();
        data.iter().for_each(|d| {
            let part = parts
                .entry(d.part.clone())
                .or_insert_with(|| Part::new(&d.part));
            part.values.push(d.measured);
        });
        parts.into_values().collect()
    }
}

#[derive(Clone)]
pub struct Part {
    pub id: String,
    pub values: Vec<f64>,
}

impl Anova for Part {
    fn mean(&self) -> f64 {
        let sum: f64 = self.values.iter().sum();
        let count: f64 = self.values.len() as f64;
        sum / count
    }
}

impl Default for Part {
    fn default() -> Self {
        Self::new("")
    }
}

impl Part {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_owned(),
            values: Vec::new(),
        }
    }
}
