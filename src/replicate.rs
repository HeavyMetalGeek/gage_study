use crate::data::{Data, FromData};
use std::collections::HashMap;

/// Constructs a `Vec<Replicate>`, consolodating all values for each part and operator combination
/// within its [values](Replicate::values) field
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

#[derive(Debug, Default, Clone)]
pub struct Replicate {
    /// Part [id](crate::part::Part::id)
    pub part: String,
    /// Operator [id](crate::operator::Operator::id)
    pub operator: String,
    /// All values for this part-operator combination
    pub values: Vec<f64>,
}

impl Replicate {
    /// Makes a new Replicate
    pub fn new(part: &str, operator: &str) -> Self {
        Self {
            part: part.to_owned(),
            operator: operator.to_owned(),
            ..Default::default()
        }
    }

    /// Factor level mean: $\bar{x}\_{ij.}$
    pub fn mean(&self) -> f64 {
        let sum: f64 = self.values.iter().sum();
        let count: f64 = self.values.len() as f64;
        sum / count
    }

    /// Sum of squared deviations from the mean of replicate measurements
    /// $$
    ///     SS\_n = pq \sum\_{k=1}^{n} \left(\bar{x}\_{ijk} - \bar{x}\_{ij.}\right)^2$$
    /// $$
    pub fn sqdiff(&self) -> f64 {
        let mean = self.mean();
        self.values.iter().map(|v| (v - mean).powi(2)).sum()
    }
}
