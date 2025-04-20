use serde::{Deserialize, Serialize};

/// Provides a conversion mechanism from [Data] structs
pub trait FromData {
    fn from_data(data: &[Data]) -> Self;
}

/// An individual data point.  This is the primary strucure used for deserialization.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Data {
    #[serde(default)]
    pub name: String,
    pub part: String,
    pub operator: String,
    pub replicate: usize,
    pub measured: f64,
    #[serde(default)]
    pub nominal: f64,
}

impl Data {
    pub fn from_raw(raw: &[u8], ext: &str) -> anyhow::Result<Vec<Self>> {
        if ext == "csv" {
            Data::from_raw_csv(raw)
        } else if ext == "json" {
            Data::from_raw_json(raw)
        } else {
            Err(anyhow::anyhow!("Invalid file type provided: {}", ext))
        }
    }

    pub fn from_raw_csv(raw: &[u8]) -> anyhow::Result<Vec<Self>> {
        match csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(raw)
            .deserialize()
            .collect::<Result<Vec<_>,_>>() {
            Ok(data) => Ok(data),
            Err(e) => {
                log::error!("Failed to parse CSV: {e:?}");
                Err(e.into())
            }
        }
    }

    pub fn from_raw_json(raw: &[u8]) -> anyhow::Result<Vec<Self>> {
        match serde_json::from_slice(raw) {
            Ok(data) => Ok(data),
            Err(e) => {
                log::error!("Failed to parse JSON: {e:?}");
                Err(e.into())
            }
        }
    }
}
