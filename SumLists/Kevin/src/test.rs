use std::fs::File;
use std::path::Path;

use serde::Deserialize;

use crate::linked_list::LinkedList;
use crate::sum_lists_forward_iterative;
use crate::sum_lists_forward_recursive;
use crate::sum_lists_forward_vec;
use crate::sum_lists_reverse;
use crate::sum_lists_reverse_vec;

#[derive(Deserialize)]
struct TestCaseDeserialized
{
    description: String,
    a: Vec<u8>,
    b: Vec<u8>,
    reverse_a: Vec<u8>,
    reverse_b: Vec<u8>,
    forward_expected: Vec<u8>,
    reverse_expected: Vec<u8>,
}

pub struct TestCase
{
    description: String,
    a: LinkedList<u8>,
    b: LinkedList<u8>,
    reverse_a: LinkedList<u8>,
    reverse_b: LinkedList<u8>,
    forward_expected: LinkedList<u8>,
    reverse_expected: LinkedList<u8>,
}

impl From<TestCaseDeserialized> for TestCase
{
    fn from(value: TestCaseDeserialized) -> Self
    {
        Self { description: value.description,
               a: value.a.into_iter().collect(),
               b: value.b.into_iter().collect(),
               reverse_a: value.reverse_a.into_iter().collect(),
               reverse_b: value.reverse_b.into_iter().collect(),
               forward_expected: value.forward_expected.into_iter().collect(),
               reverse_expected: value.reverse_expected.into_iter().collect() }
    }
}

pub fn read_test_cases() -> Vec<TestCase>
{
    let data_path = Path::new("../TestData/data.json").canonicalize().expect("Invalid path: '../TestData/data.json'");
    let open_error_msg = format!("error opening file: {}", data_path.display());
    let parse_error_msg = format!("error parsing file: {}", data_path.display());
    let file = File::open(data_path).expect(&open_error_msg);
    let test_cases: Vec<TestCaseDeserialized> = serde_json::from_reader(file).expect(&parse_error_msg);
    test_cases.into_iter().map(TestCase::from).collect()
}

#[test]
fn test_sum_lists_reverse()
{
    for case in read_test_cases()
    {
        let result = sum_lists_reverse(case.reverse_a, case.reverse_b);
        assert_eq!(result, case.reverse_expected, "sum_lists_reverse failed for: {}", case.description);
    }
}

#[test]
fn test_sum_lists_forward_iterative()
{
    for case in read_test_cases()
    {
        let result = sum_lists_forward_iterative(case.a, case.b);
        assert_eq!(result, case.forward_expected, "sum_lists_forward_iterative failed for: {}", case.description);
    }
}

#[test]
fn test_sum_lists_forward_recursive()
{
    for case in read_test_cases()
    {
        let result = sum_lists_forward_recursive(case.a, case.b);
        assert_eq!(result, case.forward_expected, "sum_lists_forward_recursive failed for: {}", case.description);
    }
}

#[test]
fn test_sum_lists_reverse_vec()
{
    for case in read_test_cases()
    {
        let result = sum_lists_reverse_vec(case.reverse_a, case.reverse_b);
        assert_eq!(result, case.reverse_expected, "sum_lists_reverse_vec failed for: {}", case.description);
    }
}

#[test]
fn test_sum_lists_forward_vec()
{
    for case in read_test_cases()
    {
        let result = sum_lists_forward_vec(case.a, case.b);
        assert_eq!(result, case.forward_expected, "sum_lists_forward_vec failed for: {}", case.description);
    }
}
