use std::default;
use std::error::Error;
use std::path::Path;

use ratatui::style::Stylize;
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::ListItem;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub enum Gender {
    #[default]
    M,
    F,
    other,
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct StudentListItem {
    // pub class: Class,
    pub id: u32,
    pub name: String,
    pub nationality: String,
    pub city: String,
    pub latitude: f32,
    pub longitude: f32,
    pub gender: Gender,
    pub age: u32,
    #[serde(rename = "english.grade")]
    pub english_grade: f32,
    #[serde(rename = "math.grade")]
    pub math_grade: f32,
    #[serde(rename = "sciences.grade")]
    pub sciences_grade: f32,
    #[serde(rename = "language.grade")]
    pub language_grade: f32,
    #[serde(rename = "portfolio.rating")]
    pub portfolio_rating: f32,
    #[serde(rename = "coverletter.rating")]
    pub coverletter_rating: f32,
    #[serde(rename = "refletter.rating")]
    pub refletter_rating: f32,
}

impl StudentListItem {
    pub fn new(
        // class: Class,
        id: u32,
        name: String,
        nationality: String,
        city: String,
        latitude: f32,
        longitude: f32,
        gender: Gender,
        age: u32,
        english_grade: f32,
        math_grade: f32,
        sciences_grade: f32,
        language_grade: f32,
        portfolio_rating: f32,
        coverletter_rating: f32,
        refletter_rating: f32,
    ) -> Self {
        Self {
            // class,
            id,
            name,
            nationality,
            city,
            latitude,
            longitude,
            gender,
            age,
            english_grade,
            math_grade,
            sciences_grade,
            language_grade,
            portfolio_rating,
            coverletter_rating,
            refletter_rating,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum Class {
    #[default]
    I,
    II,
    III,
    IV,
    V,
    VI,
    VII,
    VIII,
    IX,
    X,
    XI,
    XII,
}

impl std::fmt::Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

impl<'a> From<&'a StudentListItem> for ListItem<'a> {
    fn from(value: &'a StudentListItem) -> Self {
        let name_header = Line::from(
            format!(
                "{:<4} {:>20} {:>10} {:>30} {:>20} {:>6}",
                "ID", "LastName,FirstName", "Age", "Nationality", "City", "Sex"
            )
            .blue()
            .bold()
            .italic(),
        );
        let name_line = Line::from(
            Span::from(format!(
                "{:<4} {:>20} {:>10} {:>30} {:>20} {:>6}",
                value.id,
                &value.name,
                value.age,
                value.nationality.to_string(),
                value.city.to_string(),
                value.gender.to_string(),
            ))
            .yellow(),
        );
        let dashes = Line::from(format!("{:->100}", " "));
        let text = Text::from(vec![name_header, name_line, dashes]);
        ListItem::new(text)
    }
}

pub fn student_list_vec(path: &Path) -> Result<Vec<StudentListItem>, Box<dyn Error>> {
    if path.exists() {
        if path.is_file() {
            let mut rec: Vec<StudentListItem> = Vec::new();
            for item in csv::Reader::from_path(path)?.deserialize() {
                let result: StudentListItem = item?;
                rec.push(result);
            }

            Ok(rec)
        } else {
            Err("Unable to open file".into())
        }
    } else {
        Err("File not present".into())
    }
}
