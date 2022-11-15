use crate::data::{Data, FromData};
use crate::operator::Operator;
use crate::part::Part;
use crate::replicate::Replicate;

/// A struct for organizing data into individual components for analysis.
pub struct DataSet {
    /// The identifier for the entire DataSet
    pub name: String,
    /// The original [Data] structures
    pub data: Vec<Data>,
    /// Data values grouped by [Part]
    pub parts: Vec<Part>,
    /// Data values grouped by [Operator]
    pub operators: Vec<Operator>,
    /// Data values grouped by [Replicate]
    pub replicates: Vec<Replicate>,
    /// Flag for determining use of the part-operator interaction term
    pub use_interaction: bool,
}

/// Makes the default DataSet
impl Default for DataSet {
    fn default() -> Self {
        Self::new()
    }
}

impl DataSet {
    /// Makes a new DataSet
    pub fn new() -> Self {
        Self {
            name: String::new(),
            data: Vec::new(),
            parts: Vec::new(),
            operators: Vec::new(),
            replicates: Vec::new(),
            use_interaction: true,
        }
    }
    /// Parses data into various components needed for analysis
    pub fn from_data(name: &str, data: &[Data]) -> Self {
        Self {
            name: name.to_owned(),
            data: data.to_vec(),
            parts: Vec::from_data(data),
            operators: Vec::from_data(data),
            replicates: Vec::from_data(data),
            use_interaction: true,
        }
    }
    /// Removes the part-operator interaction from the ANOVA
    pub fn ignore_interaction(mut self) -> Self {
        self.use_interaction = false;
        self
    }
}
