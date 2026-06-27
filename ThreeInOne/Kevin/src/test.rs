use std::fs::File;
use std::path::Path;

use serde::Deserialize;

#[cfg(test)]
use crate::FixedMultiStack;
use crate::ThreeStacks;

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

#[derive(Clone, Debug, Deserialize)]
pub struct TestCase
{
    pub description: String,
    pub stack_capacity: usize,
    pub operations: Vec<Operation>,
}

pub fn read_test_cases() -> Vec<TestCase>
{
    let data_path = Path::new("../TestData/data.json").canonicalize().expect("Invalid path: '../TestData/data.json'");
    let open_error_msg = format!("error opening file: {}", data_path.display());
    let parse_error_msg = format!("error parsing file: {}", data_path.display());
    let file = File::open(data_path).expect(&open_error_msg);
    serde_json::from_reader(file).expect(&parse_error_msg)
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
type ThreeStacksCtor = fn(usize) -> Box<dyn ThreeStacks>;

#[cfg(test)]
const IMPLEMENTATIONS: &[(&str, ThreeStacksCtor)] = &[("fixed_multi_stack", make_fixed_multi_stack)];

#[test]
fn test_all_implementations()
{
    for (name, make) in IMPLEMENTATIONS
    {
        for case in read_test_cases()
        {
            let mut stack = make(case.stack_capacity);
            run_operations(&mut *stack, &case, name);
        }
    }
}
