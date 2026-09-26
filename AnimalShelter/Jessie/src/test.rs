use std::env::current_dir;
use std::fs::read_to_string;
use std::path::Path;

use serde::Deserialize;

use crate::Animal;
use crate::AnimalShelter;
use crate::Species;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Expected {
    Value(Animal),
    Nothing, // matches JSON null
}

impl Expected {
    pub const fn as_option(self) -> Option<Animal> {
        match self {
            Self::Value(animal) => Some(animal),
            Self::Nothing => None,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Operation {
    Enqueue { species: Species, id: u32 },
    DequeueAny { expected: Expected },
    DequeueDog { expected: Expected },
    DequeueCat { expected: Expected },
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
    let data_path = Path::new(path).canonicalize().unwrap_or_else(|e| {
        panic!(
            "cannot resolve test data '{path}' from {}: {e}",
            current_dir().unwrap_or_default().display()
        )
    });
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

pub fn run_operations(shelter: &mut AnimalShelter, case: &TestCase) {
    for (i, operation) in case.operations.iter().enumerate() {
        match operation {
            Operation::Enqueue { species, id } => {
                shelter.enqueue(Animal {
                    species: *species,
                    id: *id,
                });
            }
            Operation::DequeueAny { expected } => {
                assert_eq!(
                    shelter.dequeue_any(),
                    expected.as_option(),
                    "{}: op {i}: dequeue_any mismatch",
                    case.description
                );
            }
            Operation::DequeueDog { expected } => {
                assert_eq!(
                    shelter.dequeue_dog(),
                    expected.as_option(),
                    "{}: op {i}: dequeue_dog mismatch",
                    case.description
                );
            }
            Operation::DequeueCat { expected } => {
                assert_eq!(
                    shelter.dequeue_cat(),
                    expected.as_option(),
                    "{}: op {i}: dequeue_cat mismatch",
                    case.description
                );
            }
            Operation::IsEmpty { expected } => {
                assert_eq!(
                    shelter.is_empty(),
                    *expected,
                    "{}: op {i}: is_empty mismatch",
                    case.description
                );
            }
            Operation::Len { expected } => {
                assert_eq!(
                    shelter.len(),
                    *expected,
                    "{}: op {i}: len mismatch",
                    case.description
                );
            }
        }
    }
}
