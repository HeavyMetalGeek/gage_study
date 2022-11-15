use crate::data::{Data, FromData};
use std::collections::HashMap;

/// Constructs a `Vec<Part>`, consolodating all values for each Part within its
/// [values](Part::values) field and creating a sub-grouping of values by operator within its
/// [operator_values](Part::operator_values) field
impl FromData for Vec<Part> {
    fn from_data(data: &[Data]) -> Self {
        let mut parts: HashMap<String, Part> = HashMap::new();
        data.iter().for_each(|d| {
            let part = parts
                .entry(d.part.clone())
                .or_insert_with(|| Part::new(&d.part));
            part.values.push(d.measured);
            part.operator_values
                .entry(d.operator.clone())
                .or_default()
                .push(d.measured);
        });
        parts.into_values().collect()
    }
}

#[derive(Clone)]
pub struct Part {
    /// Part identifier
    pub id: String,
    /// All values for this part
    pub values: Vec<f64>,
    /// All values for this part, grouped by [Operator](crate::operator::Operator) id
    pub operator_values: HashMap<String, Vec<f64>>,
}

/// Makes the default Part with an empty string [id](Part::id)
impl Default for Part {
    fn default() -> Self {
        Self::new("")
    }
}

impl Part {
    /// Makes a new Part
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_owned(),
            values: Vec::new(),
            operator_values: HashMap::new(),
        }
    }
    /// Part mean: $\bar{x}\_{i..}$
    pub fn mean(&self) -> f64 {
        let sum: f64 = self.values.iter().sum();
        let count: f64 = self.values.len() as f64;
        sum / count
    }
    /// Sum of squared deviations from the grand mean
    /// $$
    ///     SS\_p = qn \sum\_{i=1}^{p} \left(\bar{x}\_{i..} - \bar{x}\_{...}\right)^2$$
    /// $$
    pub fn sqdiff(&self, total_mean: f64) -> f64 {
        (self.mean() - total_mean).powi(2)
    }
}
