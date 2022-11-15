use crate::data::Data;
use anyhow::{anyhow, Result};
use serde_json;
use std::ffi::OsStr;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Struct used to deserialize JSON data
pub struct JsonData {
    pub source: String,
}
/// Makes the default JsonData
impl Default for JsonData {
    fn default() -> Self {
        Self::new()
    }
}
impl JsonData {
    /// Makes a new JsonData
    pub fn new() -> Self {
        Self {
            source: String::new(),
        }
    }
    /// Checks that [source] is a file, exists, and has a `json` extension
    fn check_source(&self) -> Result<()> {
        let path = Path::new(&self.source);
        let ext = path.extension().and_then(OsStr::to_str);
        if !path.is_file() {
            return Err(anyhow!("Path must lead to a file: {}", self.source));
        } else if !path.exists() {
            return Err(anyhow!("Invalid file path: {}", self.source));
        } else if ext != Some("json") {
            return Err(anyhow!("Invalid file type: {}", self.source));
        }
        Ok(())
    }
    /// Changes the source path
    pub fn with_source(mut self, path: &str) -> Self {
        self.source = path.to_owned();
        self
    }
    /// Deserializes a list of JSON serialized [Data] objects
    pub fn deserialize_data(&mut self) -> Result<Vec<Data>> {
        self.check_source()?;
        let file = File::open(&self.source)?;
        let reader = BufReader::new(file);
        let data: Vec<Data> = serde_json::from_reader(reader)?;
        Ok(data)
    }
}
