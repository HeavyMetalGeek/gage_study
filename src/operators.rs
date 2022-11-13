use crate::anova::Anova;
use crate::data::{Data, FromData};
use std::collections::HashMap;

impl FromData for Vec<Operator> {
    fn from_data(data: &[Data]) -> Self {
        let mut operators: HashMap<String, Operator> = HashMap::new();
        data.iter().for_each(|d| {
            let operator = operators
                .entry(d.operator.clone())
                .or_insert_with(|| Operator::new(&d.operator));
            operator.values.push(d.measured);
        });
        operators.into_values().collect()
    }
}

#[derive(Clone)]
pub struct Operator {
    pub id: String,
    pub values: Vec<f64>,
}

impl Anova for Operator {
    fn mean(&self) -> f64 {
        let sum: f64 = self.values.iter().sum();
        let count: f64 = self.values.len() as f64;
        sum / count
    }
}

impl Default for Operator {
    fn default() -> Self {
        Self::new("")
    }
}

impl Operator {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_owned(),
            values: Vec::new(),
        }
    }
}
