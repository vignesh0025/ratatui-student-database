use crate::app::app_structs::StudentListItem as StudentListEntry;
use clap::builder::Str;
use csv;
use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::{
    fs::OpenOptions,
    io::{Write},
    path::{Path},
};

#[derive(Default)]
pub struct DataModel {
    pub data_items: Vec<StudentListEntry>,
    active: bool,
    filter_string: Option<String>,
}

#[derive(Debug, PartialEq, Default)]
pub enum DataType {
    MsgPackBin,
    #[default]
    CsvBin,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct MsgPackContainer {
    rec: Vec<StudentListEntry>,
}

impl DataModel {
    pub fn new(path: &Path, data_type: DataType) -> Result<Self, Box<dyn Error>> {
        let mut rec: Vec<StudentListEntry>;
        if path.exists() && path.is_file() {
            match data_type {
                DataType::MsgPackBin => {
                    let file = OpenOptions::new().read(true).open(path)?;
                    let mut deserializer = rmp_serde::Deserializer::new(file);
                    let msgpack_data: MsgPackContainer =
                        MsgPackContainer::deserialize(&mut deserializer).unwrap();
                    rec = msgpack_data.rec;
                }
                DataType::CsvBin => {
                    rec = Vec::new();
                    for item in csv::Reader::from_path(path)?.deserialize() {
                        let result: StudentListEntry = item?;
                        rec.push(result);
                    }
                }
            }
        } else {
            panic!("Database not found");
        }

        Ok(Self {
            data_items: rec,
            active: true,
            filter_string: None
        })
    }

    pub fn set_filter<T>(&mut self, filter_string: T) where T: AsRef<str> {
        let filter_string = filter_string.as_ref();
        self.filter_string = Some(filter_string.into());
    }

    pub fn clear_filter(&mut self) {
        self.filter_string = None;
    }

    pub fn filter(&self) -> Vec<&StudentListEntry> {
        if let Some(s) = &self.filter_string {
                // let s: &str = s.as_ref();
                /* We directly converted Vec<StudentListEntry> to Vec<&StudentListEntry> */
                self.data_items
                .iter()
                .filter(|f| 
                    f.name.contains(s)
                        || f.city.contains(s)
                        || f.nationality.contains(s)
                ).collect()
        } else {
            self.data_items.iter().collect()
        }
    }

    pub fn save(&mut self, data_type: DataType, path: &Path) -> Result<(), Box<dyn Error>> {
        if self.active {
            match data_type {
                DataType::CsvBin => {
                    let mut wtr = csv::Writer::from_path(path)?;
                    for entry in self.data_items.iter() {
                        wtr.serialize(entry)?;
                    }
                    wtr.flush()?;
                },
                DataType::MsgPackBin => {
                    let mut buf = Vec::new();
                    let data = MsgPackContainer {
                        rec: self.data_items.clone()
                    };
                    data.serialize(&mut Serializer::new(&mut buf))?;
                    let mut file = OpenOptions::new().write(true).open(path)?;
                    file.write(&buf)?;
                    file.flush()?;
                }
            }
        }

        Ok(())
    }
}
