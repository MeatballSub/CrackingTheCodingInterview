use std::env::current_dir;
use std::fs::read_to_string;
use std::path::Path;

use serde::Deserialize;

use crate::NamedStack;
use crate::SortableStack;
use crate::insertion_sort_stack::InsertionSortStack;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Expected
{
    Value(i32),
    Nothing, // matches JSON null
}

impl Expected
{
    #[must_use]
    pub const fn as_option(self) -> Option<i32>
    {
        match self
        {
            Self::Value(value) => Some(value),
            Self::Nothing => None,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Operation
{
    Push
    {
        value: i32,
    },
    Pop
    {
        expected: Expected,
    },
    Peek
    {
        expected: Expected,
    },
    IsEmpty
    {
        expected: bool,
    },
    Len
    {
        expected: usize,
    },
    Sort,
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

pub fn run_operations<S: SortableStack + ?Sized>(stack: &mut S, case: &TestCase, label: &str)
{
    for (i, operation) in case.operations.iter().enumerate()
    {
        match operation
        {
            Operation::Push { value } =>
            {
                stack.push(*value);
            }
            Operation::Pop { expected } =>
            {
                assert_eq!(stack.pop(), expected.as_option(), "{label} / {}: op {i}: pop mismatch", case.description);
            }
            Operation::Peek { expected } =>
            {
                assert_eq!(stack.peek(), expected.as_option(), "{label} / {}: op {i}: peek mismatch", case.description);
            }
            Operation::IsEmpty { expected } =>
            {
                assert_eq!(stack.is_empty(), *expected, "{label} / {}: op {i}: is_empty mismatch", case.description);
            }
            Operation::Len { expected } =>
            {
                assert_eq!(stack.len(), *expected, "{label} / {}: op {i}: len mismatch", case.description);
            }
            Operation::Sort =>
            {
                stack.sort();
            }
        }
    }
}

pub type SortableStackCtor = fn() -> Box<dyn SortableStack>;

fn make<S: NamedStack + 'static>() -> Box<dyn SortableStack> { Box::new(S::default()) }

macro_rules! implementations {
    ($($stack:ty),+ $(,)?) =>
    {
        pub const IMPLEMENTATIONS: &[(&str, SortableStackCtor)] = &[$((<$stack as NamedStack>::NAME, make::<$stack> as SortableStackCtor)),+];
    };
}

implementations!(InsertionSortStack);

#[test]
fn test_all_implementations()
{
    let shared_cases = read_test_cases();
    for &(name, make) in IMPLEMENTATIONS
    {
        for case in &shared_cases
        {
            let mut stack = make();
            run_operations(&mut *stack, case, name);
        }
    }
}
