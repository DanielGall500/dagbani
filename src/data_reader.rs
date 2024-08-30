use csv::{self};
use serde::Deserialize;
use std::error::Error;

fn read_csv(path: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .escape(Some(b'\\'))
        .from_path(path)?;

    let mut records = Vec::new();
    for result in rdr.deserialize() {
        let record: Record = result?;
        records.push(record);
    }
    Ok(records)
}

#[derive(Debug, Deserialize)]
pub struct Record {
    pub phonetic_rep: String,
    pub english_translation: String,
    pub pos: String,
    pub source: Option<String>,
    pub structure: String,
}

pub struct Dataset {
    pub path: String,
    pub data: Vec<Record>,
}

impl Dataset {
    pub fn new(path: String) -> Result<Dataset, Box<dyn Error>> {
        let data = read_csv(&path)?;
        Ok(Self {
            path: path,
            data: data,
        })
    }

    pub fn get_data(&self) -> Result<&Vec<Record>, Box<dyn Error>> {
        Ok(&self.data)
    }
}