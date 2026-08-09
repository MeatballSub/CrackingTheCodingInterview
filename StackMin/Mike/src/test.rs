use std::fs::read_to_string;
use std::path::Path;

use serde::Deserialize;

use crate::MinStack;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Operation {
    Push { value: i32 },
    Pop { expected: Option<i32> },
    Min { expected: Option<i32> },
    IsEmpty { expected: bool },
}

#[derive(Clone, Debug, Deserialize)]
pub struct TestCase {
    pub description: String,
    pub operations: Vec<Operation>,
}

pub fn read_test_cases() -> Vec<TestCase> {
    let data_path = Path::new("../TestData/data.json")
        .canonicalize()
        .expect("Invalid path: '../TestData/data.json'");
    let open_error_msg = format!("error opening file: {}", data_path.display());
    let parse_error_msg = format!("error parsing file: {}", data_path.display());
    let text = read_to_string(&data_path).expect(&open_error_msg);
    serde_json::from_str(&text).expect(&parse_error_msg)
}

pub fn run_operations(stack: &mut MinStack, case: &TestCase) {
    for (i, operation) in case.operations.iter().enumerate() {
        match operation {
            Operation::Push { value } => {
                stack.push(*value);
            }
            Operation::Pop { expected } => {
                assert_eq!(
                    stack.pop(),
                    *expected,
                    "{}: op {i}: pop mismatch",
                    case.description
                );
            }
            Operation::Min { expected } => {
                assert_eq!(
                    stack.min(),
                    *expected,
                    "{}: op {i}: min mismatch",
                    case.description
                );
            }
            Operation::IsEmpty { expected } => {
                assert_eq!(
                    stack.is_empty(),
                    *expected,
                    "{}: op {i}: is_empty mismatch",
                    case.description
                );
            }
        }
    }
}
