use std::env::current_dir;
use std::fs::read_to_string;
use std::path::Path;

use serde::Deserialize;

use crate::Animal;
use crate::AnimalShelter;
use crate::NamedShelter;
use crate::Species;
use crate::two_queue_shelter::TwoQueueShelter;

/// A nullable expected value. Modeling this as an untagged enum rather than an
/// `Option<Animal>` keeps serde from supplying an implicit default, so an
/// omitted `expected` key is a parse error instead of a silent `None`.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Expected
{
    Value(Animal),
    Nothing, // matches JSON null
}

impl Expected
{
    #[must_use]
    pub const fn as_option(self) -> Option<Animal>
    {
        match self
        {
            Self::Value(animal) => Some(animal),
            Self::Nothing => None,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Operation
{
    Enqueue
    {
        species: Species, id: u32
    },
    DequeueAny
    {
        expected: Expected
    },
    DequeueDog
    {
        expected: Expected
    },
    DequeueCat
    {
        expected: Expected
    },
    IsEmpty
    {
        expected: bool
    },
    Len
    {
        expected: usize
    },
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TestCase
{
    pub description: String,
    pub operations: Vec<Operation>,
}

const SHARED_TEST_DATA: &str = "../TestData/data.json";

pub fn read_test_data(path: &str) -> Vec<TestCase>
{
    let data_path = Path::new(path).canonicalize()
                                   .unwrap_or_else(|e| panic!("cannot resolve test data '{path}' from {}: {e}", current_dir().unwrap_or_default().display()));
    let open_error_msg = format!("error opening file: {}", data_path.display());
    let parse_error_msg = format!("error parsing file: {}", data_path.display());
    let text = read_to_string(&data_path).expect(&open_error_msg);
    serde_json::from_str(&text).expect(&parse_error_msg)
}

pub fn read_test_cases() -> Vec<TestCase> { read_test_data(SHARED_TEST_DATA) }

const BENCH_TEST_DATA: &str = "../TestData/benchmark.json";

pub fn read_bench_cases() -> Vec<TestCase> { read_test_data(BENCH_TEST_DATA) }

pub fn run_operations<S: AnimalShelter + ?Sized>(shelter: &mut S, case: &TestCase, label: &str)
{
    for (i, operation) in case.operations.iter().enumerate()
    {
        match operation
        {
            Operation::Enqueue { species, id } =>
            {
                shelter.enqueue(Animal { species: *species, id: *id });
            }
            Operation::DequeueAny { expected } =>
            {
                assert_eq!(shelter.dequeue_any(), expected.as_option(), "{label} / {}: op {i}: dequeue_any mismatch", case.description);
            }
            Operation::DequeueDog { expected } =>
            {
                assert_eq!(shelter.dequeue_dog(), expected.as_option(), "{label} / {}: op {i}: dequeue_dog mismatch", case.description);
            }
            Operation::DequeueCat { expected } =>
            {
                assert_eq!(shelter.dequeue_cat(), expected.as_option(), "{label} / {}: op {i}: dequeue_cat mismatch", case.description);
            }
            Operation::IsEmpty { expected } =>
            {
                assert_eq!(shelter.is_empty(), *expected, "{label} / {}: op {i}: is_empty mismatch", case.description);
            }
            Operation::Len { expected } =>
            {
                assert_eq!(shelter.len(), *expected, "{label} / {}: op {i}: len mismatch", case.description);
            }
        }
    }
}

pub type AnimalShelterCtor = fn() -> Box<dyn AnimalShelter>;

fn make<S: NamedShelter + 'static>() -> Box<dyn AnimalShelter> { Box::new(S::default()) }

/// To add an implementation: write a type in its own module that implements
/// `AnimalShelter` and `NamedShelter`, then add it to this list and to the
/// benchmark's `bench_animal_shelter_impls!` call. Both registries derive their
/// labels from `NamedShelter::NAME`, so a label can never name the wrong type.
macro_rules! implementations {
    ($($shelter:ty),+ $(,)?) =>
    {
        pub const IMPLEMENTATIONS: &[(&str, AnimalShelterCtor)] = &[$((<$shelter as NamedShelter>::NAME, make::<$shelter> as AnimalShelterCtor)),+];
    };
}

implementations!(TwoQueueShelter);

#[test]
fn test_all_implementations()
{
    let shared_cases = read_test_cases();
    for &(name, make) in IMPLEMENTATIONS
    {
        for case in &shared_cases
        {
            let mut shelter = make();
            run_operations(&mut *shelter, case, name);
        }
    }
}
