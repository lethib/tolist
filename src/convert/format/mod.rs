use crate::convert::{
    ConvertError, Field,
    format::{csv::CSV, json::JSON, python::PYTHON, ruby::RUBY, sql::SQL},
};

pub mod csv;
pub mod json;
pub mod python;
pub mod ruby;
pub mod sql;

#[derive(Debug)]
pub struct Format {
    pub name: &'static str,
    pub open: &'static str,
    pub close: &'static str,
    pub sep: &'static str,
    pub quote: fn(&str) -> String,
}

impl Format {
    pub fn parse(name: &str) -> Result<Self, ConvertError> {
        match name.trim().to_lowercase().as_str() {
            "json" | "js" | "javascript" | "go" => Ok(JSON),
            "sql" => Ok(SQL),
            "python" | "py" => Ok(PYTHON),
            "ruby" => Ok(RUBY),
            "csv" => Ok(CSV),
            _ => Err(ConvertError::UnknownFormat(name.to_string())),
        }
    }

    pub fn render(&self, fields: Vec<Field>, all_numbers: bool) -> String {
        let mut res = String::from(self.open);
        for (i, field) in fields.iter().enumerate() {
            if i > 0 {
                res.push_str(self.sep);
            }

            if all_numbers {
                res.push_str(&field.val);
            } else {
                res.push_str(&(self.quote)(&field.val));
            }
        }

        res.push_str(self.close);

        res
    }
}
