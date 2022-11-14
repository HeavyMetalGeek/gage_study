use crate::data::{Data, FromData};
use crate::operators::Operator;
use crate::parts::Part;
use crate::replicates::Replicate;

pub struct DataSet {
    pub name: String,
    pub data: Vec<Data>,
    pub parts: Vec<Part>,
    pub operators: Vec<Operator>,
    pub replicates: Vec<Replicate>,
    pub use_interaction: bool,
}

impl Default for DataSet {
    fn default() -> Self {
        Self::new()
    }
}

impl DataSet {
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
    pub fn ignore_interaction(mut self) -> Self {
        self.use_interaction = false;
        self
    }
}
