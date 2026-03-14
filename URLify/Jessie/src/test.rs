use std::fs::File;
use std::path::Path;

use serde::Deserialize;

#[derive(Debug)]
pub struct TestCase {
    pub input_string: Vec<char>,
    pub input_true_length: usize,
    pub answer: Vec<char>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct ParsedCase {
    pub input_string: String,
    pub input_true_length: usize,
    pub answer: String,
}

impl From<&ParsedCase> for TestCase {
    fn from(value: &ParsedCase) -> Self {
        Self {
            input_string: value.input_string.chars().collect(),
            input_true_length: value.input_true_length,
            answer: value.answer.chars().collect(),
        }
    }
}

pub fn read_test_cases() -> Vec<TestCase> {
    let data_path = Path::new("../TestData/data.json")
        .canonicalize()
        .expect("Invalid path: '../TestData/data.json'");
    let open_error_msg = format!("error opening file: {}", data_path.display());
    let parse_error_msg = format!("error parsing file: {}", data_path.display());
    let file = File::open(data_path).expect(&open_error_msg);
    serde_json::from_reader::<std::fs::File, Vec<ParsedCase>>(file)
        .expect(&parse_error_msg)
        .iter()
        .map(|c| c.into())
        .collect()
}
