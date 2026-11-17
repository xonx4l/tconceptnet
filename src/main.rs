use std::fmt;
use serde::Deserialize;


#[derive(Debug)]
struct Detail {
    relation: String,
    start: String,
    end: String,
    weight: f64,
    surface_text: Option<String>, // many lines don't have readable sentences so it must be
    // allowed to be absent.
}

#[derive(Deserialize)]
struct Meta {
    weight: Option<f64>,
    surface_text: Option<String>,
}

#[derive(Debug, PartialEq)]
enum ParseError {
    TooFewColumns,
    EmptyField,
    BadJson,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::TooFewColumns => write!(f,"Less columns than expected"),
            ParseError::EmptyField => write!(f, "Field is empty"),
            ParseError::BadJson => write!(f,"Invalid JSON"),
        }
    }
}

fn input(text: &str) -> Result<Detail, ParseError> {
    // SPLIT THE LINE ON TABS INTO AT MOST 5 COLUMNS  
    let columns : Vec<&str> = text.splitn(5 ,'\t').collect();
    if columns.len() < 5 {
        return Err(ParseError::TooFewColumns);
    }

    //check there are five and the relation start and end are not empty   
    let relation = columns[1];
    let start = columns[2];
    let end = columns[3];
    if relation.is_empty() || start.is_empty() || end.is_empty() {
        return Err(ParseError::EmptyField);
  }

   Ok (Detail {
         relation : relation.to_string(),
         start : start.to_string(),
         end : end.to_string(),
         weight : Meta::weight.unwrap_or(1.0),
         surface_text : Meta::surface_text,
   })
}

fn main () {
    todo!()
}




