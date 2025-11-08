use std::fs::File;
use std::path::Path;

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct TestCase {
    pub input1: String,
    pub input2: String,
    pub answer: bool,
}

pub fn read_test_cases() -> Vec<TestCase> {
    let data_path = Path::new("../TestData/data.json")
        .canonicalize()
        .expect("Invalid path: '../TestData/data.json'");
    let open_error_msg = format!("error opening file: {}", data_path.display());
    let parse_error_msg = format!("error parsing file: {}", data_path.display());
    let file = File::open(data_path).expect(&open_error_msg);
    serde_json::from_reader(file).expect(&parse_error_msg)
}
