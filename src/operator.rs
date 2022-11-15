use crate::data::{Data, FromData};
use std::collections::HashMap;

/// Constructs a `Vec<Operator>`, consolodating all values for each Operator within its
/// [values](Operator::values) field and creating a sub-grouping of values by operator within its
/// [operator_values](Operator::part_values) field
impl FromData for Vec<Operator> {
    fn from_data(data: &[Data]) -> Self {
        let mut operators: HashMap<String, Operator> = HashMap::new();
        data.iter().for_each(|d| {
            let operator = operators
                .entry(d.operator.clone())
                .or_insert_with(|| Operator::new(&d.operator));
            operator.values.push(d.measured);
            operator
                .part_values
                .entry(d.part.clone())
                .or_default()
                .push(d.measured);
        });
        operators.into_values().collect()
    }
}

#[derive(Clone)]
pub struct Operator {
    /// Operator identifier
    pub id: String,
    /// All values for this operator
    pub values: Vec<f64>,
    /// All values for this operator, grouped by [Part](crate::part::Part) id
    pub part_values: HashMap<String, Vec<f64>>,
}

/// Makes the default Operator with an empty [id](Operator::id)
impl Default for Operator {
    fn default() -> Self {
        Self::new("")
    }
}

impl Operator {
    /// Makes a new Operator
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_owned(),
            values: Vec::new(),
            part_values: HashMap::new(),
        }
    }
    /// Part mean: $\bar{x}\_{.j.}$
    pub fn mean(&self) -> f64 {
        let sum: f64 = self.values.iter().sum();
        let count: f64 = self.values.len() as f64;
        sum / count
    }
    /// Sum of squared deviations from the grand mean
    /// $$
    ///     SS\_p = pn \sum\_{j=1}^{q} \left(\bar{x}\_{.j.} - \bar{x}\_{...}\right)^2$$
    /// $$
    pub fn sqdiff(&self, total_mean: f64) -> f64 {
        (self.mean() - total_mean).powi(2)
    }
}
