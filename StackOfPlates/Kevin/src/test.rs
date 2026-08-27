use std::fs::read_to_string;
use std::path::Path;

use serde::Deserialize;

use crate::SetOfStacks;
use crate::flat_stacks::FlatStacks;
use crate::no_rollover_stacks::NoRolloverStacks;
use crate::rollover_stacks::RolloverStacks;

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
    PopAt
    {
        index: usize, expected: Option<i32>
    },
    Peek
    {
        expected: Option<i32>
    },
    IsEmpty
    {
        expected: bool
    },
    StackCount
    {
        expected: usize
    },
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TestCase
{
    pub description: String,
    pub capacity: usize,
    pub operations: Vec<Operation>,
}

const SHARED_TEST_DATA: &str = "../TestData/data.json";

pub fn read_test_data(path: &str) -> Vec<TestCase>
{
    let data_path = Path::new(path).canonicalize().unwrap_or_else(|_| panic!("Invalid path: '{path}'"));
    let open_error_msg = format!("error opening file: {}", data_path.display());
    let parse_error_msg = format!("error parsing file: {}", data_path.display());
    let text = read_to_string(&data_path).expect(&open_error_msg);
    serde_json::from_str(&text).expect(&parse_error_msg)
}

pub fn read_test_cases() -> Vec<TestCase> { read_test_data(SHARED_TEST_DATA) }

const BENCH_TEST_DATA: &str = "../TestData/benchmark.json";

pub fn read_bench_cases() -> Vec<TestCase> { read_test_data(BENCH_TEST_DATA) }

pub fn run_operations<S: SetOfStacks + ?Sized>(stack: &mut S, case: &TestCase, label: &str)
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
            Operation::PopAt { index, expected } =>
            {
                assert_eq!(stack.pop_at(*index), *expected, "{label} / {}: op {i}: pop_at({index}) mismatch", case.description);
            }
            Operation::Peek { expected } =>
            {
                assert_eq!(stack.peek(), *expected, "{label} / {}: op {i}: peek mismatch", case.description);
            }
            Operation::IsEmpty { expected } =>
            {
                assert_eq!(stack.is_empty(), *expected, "{label} / {}: op {i}: is_empty mismatch", case.description);
            }
            Operation::StackCount { expected } =>
            {
                assert_eq!(stack.stack_count(), *expected, "{label} / {}: op {i}: stack_count mismatch", case.description);
            }
        }
    }
}

fn make_no_rollover_stacks(capacity: usize) -> Box<dyn SetOfStacks> { Box::new(NoRolloverStacks::new(capacity)) }

fn make_rollover_stacks(capacity: usize) -> Box<dyn SetOfStacks> { Box::new(RolloverStacks::new(capacity)) }

fn make_flat_stacks(capacity: usize) -> Box<dyn SetOfStacks> { Box::new(FlatStacks::new(capacity)) }

pub type SetOfStacksCtor = fn(usize) -> Box<dyn SetOfStacks>;

pub const IMPLEMENTATIONS: &[(&str, SetOfStacksCtor, &str)] = &[("no_rollover_stacks", make_no_rollover_stacks, "TestData/no_rollover.json"),
                                                                ("rollover_stacks", make_rollover_stacks, "TestData/rollover.json"),
                                                                ("flat_stacks", make_flat_stacks, "TestData/rollover.json")];

#[test]
fn test_all_implementations()
{
    let shared_cases = read_test_cases();
    for &(name, make, contract_data) in IMPLEMENTATIONS
    {
        let contract_cases = read_test_data(contract_data);
        assert!(!contract_cases.is_empty(), "{name}: {contract_data} holds no cases");

        for case in shared_cases.iter().chain(contract_cases.iter())
        {
            let mut stack = make(case.capacity);
            run_operations(&mut *stack, case, name);
        }
    }
}
