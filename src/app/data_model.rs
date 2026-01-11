use std::{collections::btree_map::IterMut, default, fs::OpenOptions, io::{Read, Write}, path::{self, Path}};
use csv;
use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::borrow::Cow;
use crate::app::app_structs::StudentListItem as StudentListEntry;

#[derive(Default)]
pub struct DataModel {
    pub data_items: Vec<StudentListEntry>,
    active: bool
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
                    let mut buf = Vec::new();
                    let mut file = OpenOptions::new().read(true).open(path)?;
                    file.read_to_end(&mut buf)?;
                    let mut deserializer = rmp_serde::Deserializer::new(&buf[..]);
                    let msgpack_data: MsgPackContainer = MsgPackContainer::deserialize(&mut deserializer).unwrap();
                    rec = msgpack_data.rec;
                },
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

        Ok(Self { data_items: rec, active: true })
    }

    pub fn filter(&self, filter_string: &str) {
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
