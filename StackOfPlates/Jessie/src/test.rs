use std::fs::read_to_string;
use std::path::Path;

use serde::Deserialize;

use crate::SetOfStacks;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Operation {
    Push { value: i32 },
    Pop { expected: Option<i32> },
    PopAt { index: usize, expected: Option<i32> },
    Peek { expected: Option<i32> },
    IsEmpty { expected: bool },
    StackCount { expected: usize },
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TestCase {
    pub description: String,
    pub capacity: usize,
    pub operations: Vec<Operation>,
}

fn read_test_data(path: &str) -> Vec<TestCase> {
    let data_path = Path::new(path)
        .canonicalize()
        .unwrap_or_else(|_| panic!("Invalid path: '{path}'"));
    let open_error_msg = format!("error opening file: {}", data_path.display());
    let parse_error_msg = format!("error parsing file: {}", data_path.display());
    let text = read_to_string(&data_path).expect(&open_error_msg);
    serde_json::from_str(&text).expect(&parse_error_msg)
}

pub fn read_test_cases() -> Vec<TestCase> {
    read_test_data("../TestData/data.json")
}

pub fn read_bench_cases() -> Vec<TestCase> {
    read_test_data("../TestData/benchmark.json")
}

pub fn run_operations(stack: &mut SetOfStacks, case: &TestCase) {
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
            Operation::PopAt { index, expected } => {
                assert_eq!(
                    stack.pop_at(*index),
                    *expected,
                    "{}: op {i}: pop_at({index}) mismatch",
                    case.description
                );
            }
            Operation::Peek { expected } => {
                assert_eq!(
                    stack.peek(),
                    *expected,
                    "{}: op {i}: peek mismatch",
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
            Operation::StackCount { expected } => {
                assert_eq!(
                    stack.stack_count(),
                    *expected,
                    "{}: op {i}: stack_count mismatch",
                    case.description
                );
            }
        }
    }
}
