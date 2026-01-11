use std::path::Path;
use csv;
use std::error::Error;

use crate::app::app_structs::StudentListItem as StudentListEntry;

struct DataMode<'a> {
    display_items: &'a Vec<StudentListEntry>,
    data_items: Vec<StudentListEntry>,
}

#[derive(Debug, PartialEq)]
enum DataType {
    MsgPackBin,
    CsvBin,
}

impl<'a> DataMode<'a> {
    fn new(path: &Path, data_type: DataType) -> Result<Self, Box<dyn Error>> {
        let mut rec: Vec<StudentListEntry> = Vec::new();
        if path.exists() && path.is_file() {
            match data_type {
                DataType::MsgPackBin => {}
                DataType::CsvBin => {
                    for item in csv::Reader::from_path(path)?.deserialize() {
                        let result: StudentListEntry = item?;
                        rec.push(result);
                    }
                }
            }
        } else {
            panic!("Database not found");
        }

        Ok(Self { display_items: &rec, data_items: rec })
    }
}
