use std::fs::read_to_string;
use std::path::Path;

use serde::Deserialize;

use crate::MyQueue;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Operation {
    Enqueue { value: i32 },
    Dequeue { expected: Option<i32> },
    Peek { expected: Option<i32> },
    IsEmpty { expected: bool },
    Len { expected: usize },
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TestCase {
    pub description: String,
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

pub fn run_operations(queue: &mut MyQueue, case: &TestCase) {
    for (i, operation) in case.operations.iter().enumerate() {
        match operation {
            Operation::Enqueue { value } => {
                queue.enqueue(*value);
            }
            Operation::Dequeue { expected } => {
                assert_eq!(
                    queue.dequeue(),
                    *expected,
                    "{}: op {i}: dequeue mismatch",
                    case.description
                );
            }
            Operation::Peek { expected } => {
                assert_eq!(
                    queue.peek(),
                    *expected,
                    "{}: op {i}: peek mismatch",
                    case.description
                );
            }
            Operation::IsEmpty { expected } => {
                assert_eq!(
                    queue.is_empty(),
                    *expected,
                    "{}: op {i}: is_empty mismatch",
                    case.description
                );
            }
            Operation::Len { expected } => {
                assert_eq!(
                    queue.len(),
                    *expected,
                    "{}: op {i}: len mismatch",
                    case.description
                );
            }
        }
    }
}
