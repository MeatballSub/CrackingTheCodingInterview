use std::fs::File;
use std::path::Path;

use serde::Deserialize;

use crate::ThreeStacks;
#[cfg(test)]
use crate::fixed_multi_stack::FixedMultiStack;
#[cfg(test)]
use crate::flexible_multi_stack::FlexibleMultiStack;

#[derive(Clone, Debug, Deserialize)]
pub enum Operation
{
    Push
    {
        stack: usize, value: i32, ok: bool
    },
    Pop
    {
        stack: usize, expected: Option<i32>
    },
    Peek
    {
        stack: usize, expected: Option<i32>
    },
    IsEmpty
    {
        stack: usize, expected: bool
    },
}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CapacityModel
{
    #[default]
    Any,
    PerStack,
    Total,
}

impl CapacityModel
{
    pub fn applies_to(self, implementation: CapacityModel) -> bool { self == CapacityModel::Any || self == implementation }
}

#[derive(Clone, Debug, Deserialize)]
pub struct TestCase
{
    pub description: String,
    pub stack_capacity: usize,
    #[serde(default)]
    pub capacity_model: CapacityModel,
    pub operations: Vec<Operation>,
}

const SHARED_TEST_DATA: &str = "../TestData/data.json";

#[cfg(test)]
const LOCAL_TEST_DATA: &str = "TestData/data.json";

fn read_test_data(path: &str) -> Vec<TestCase>
{
    let data_path = Path::new(path).canonicalize().unwrap_or_else(|_| panic!("Invalid path: '{path}'"));
    let open_error_msg = format!("error opening file: {}", data_path.display());
    let parse_error_msg = format!("error parsing file: {}", data_path.display());
    let file = File::open(data_path).expect(&open_error_msg);
    serde_json::from_reader(file).expect(&parse_error_msg)
}

pub fn read_test_cases() -> Vec<TestCase> { read_test_data(SHARED_TEST_DATA) }

#[cfg(test)]
fn read_all_test_cases() -> Vec<TestCase>
{
    let mut cases = read_test_data(SHARED_TEST_DATA);
    cases.extend(read_test_data(LOCAL_TEST_DATA));
    cases
}

pub fn run_operations(stack: &mut dyn ThreeStacks, case: &TestCase, label: &str)
{
    for (i, operation) in case.operations.iter().enumerate()
    {
        match operation
        {
            Operation::Push { stack: s, value, ok } =>
            {
                assert_eq!(stack.push(*s, *value).is_ok(), *ok, "{label} / {}: op {i}: push result mismatch", case.description);
            }
            Operation::Pop { stack: s, expected } =>
            {
                assert_eq!(stack.pop(*s), *expected, "{label} / {}: op {i}: pop mismatch", case.description);
            }
            Operation::Peek { stack: s, expected } =>
            {
                assert_eq!(stack.peek(*s), *expected, "{label} / {}: op {i}: peek mismatch", case.description);
            }
            Operation::IsEmpty { stack: s, expected } =>
            {
                assert_eq!(stack.is_empty(*s), *expected, "{label} / {}: op {i}: is_empty mismatch", case.description);
            }
        }
    }
}

#[cfg(test)]
fn make_fixed_multi_stack(stack_capacity: usize) -> Box<dyn ThreeStacks> { Box::new(FixedMultiStack::new(stack_capacity)) }

#[cfg(test)]
fn make_flexible_multi_stack(stack_capacity: usize) -> Box<dyn ThreeStacks> { Box::new(FlexibleMultiStack::new(stack_capacity)) }

#[cfg(test)]
type ThreeStacksCtor = fn(usize) -> Box<dyn ThreeStacks>;

#[cfg(test)]
const IMPLEMENTATIONS: &[(&str, ThreeStacksCtor, CapacityModel)] = &[("fixed_multi_stack", make_fixed_multi_stack, CapacityModel::PerStack),
                                                                     ("flexible_multi_stack", make_flexible_multi_stack, CapacityModel::Total)];

#[test]
fn test_all_implementations()
{
    let cases = read_all_test_cases();
    for (name, make, model) in IMPLEMENTATIONS
    {
        let mut ran = 0;
        let mut model_specific = 0;
        for case in cases.iter().filter(|case| case.capacity_model.applies_to(*model))
        {
            let mut stack = make(case.stack_capacity);
            run_operations(&mut *stack, case, name);
            ran += 1;
            if case.capacity_model == *model
            {
                model_specific += 1;
            }
        }
        assert_ne!(ran, 0, "{name}: no test cases apply to {model:?}");
        assert_ne!(model_specific, 0, "{name}: no test case exercises the {model:?} capacity model");
    }
}
