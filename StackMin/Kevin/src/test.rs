use std::fs::read_to_string;
use std::path::Path;

use serde::Deserialize;

use crate::MinStack;
use crate::auxiliary_min_stack::AuxiliaryMinStack;
use crate::paired_min_stack::PairedMinStack;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Operation
{
    Push
    {
        value: i32
    },
    Pop
    {
        expected: Option<i32>
    },
    Min
    {
        expected: Option<i32>
    },
    IsEmpty
    {
        expected: bool
    },
}

#[derive(Clone, Debug, Deserialize)]
pub struct TestCase
{
    pub description: String,
    pub operations: Vec<Operation>,
}

const SHARED_TEST_DATA: &str = "../TestData/data.json";

fn read_test_data(path: &str) -> Vec<TestCase>
{
    let data_path = Path::new(path).canonicalize().unwrap_or_else(|_| panic!("Invalid path: '{path}'"));
    let open_error_msg = format!("error opening file: {}", data_path.display());
    let parse_error_msg = format!("error parsing file: {}", data_path.display());
    let text = read_to_string(&data_path).expect(&open_error_msg);
    serde_json::from_str(&text).expect(&parse_error_msg)
}

pub fn read_test_cases() -> Vec<TestCase> { read_test_data(SHARED_TEST_DATA) }

pub fn run_operations<S: MinStack + ?Sized>(stack: &mut S, case: &TestCase, label: &str)
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
                assert_eq!(stack.pop(), *expected, "{label} / {}: op {i}: pop mismatch", case.description);
            }
            Operation::Min { expected } =>
            {
                assert_eq!(stack.min(), *expected, "{label} / {}: op {i}: min mismatch", case.description);
            }
            Operation::IsEmpty { expected } =>
            {
                assert_eq!(stack.is_empty(), *expected, "{label} / {}: op {i}: is_empty mismatch", case.description);
            }
        }
    }
}

fn make_paired_min_stack() -> Box<dyn MinStack> { Box::new(PairedMinStack::new()) }

fn make_auxiliary_min_stack() -> Box<dyn MinStack> { Box::new(AuxiliaryMinStack::new()) }

pub type MinStackCtor = fn() -> Box<dyn MinStack>;

/// The one registry of implementations, shared by the test and the benchmark.
///
/// To add an implementation: write it in its own module, declare that module in
/// `lib.rs`, add one entry here, and add one arm to the benchmark's
/// `bench_stack_min_impls!` call. The benchmark asserts at compile time that its
/// arm count matches this table, so an implementation cannot be tested but left
/// unbenchmarked (or the reverse) without a build failure.
pub const IMPLEMENTATIONS: &[(&str, MinStackCtor)] = &[("paired_min_stack", make_paired_min_stack), ("auxiliary_min_stack", make_auxiliary_min_stack)];

#[test]
fn test_all_implementations()
{
    let cases = read_test_cases();
    for &(name, make) in IMPLEMENTATIONS
    {
        for case in &cases
        {
            let mut stack = make();
            run_operations(&mut *stack, case, name);
        }
    }
}
