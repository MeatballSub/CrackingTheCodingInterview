use std::fs::File;
use std::path::Path;

use serde::Deserialize;

use crate::linked_list::LinkedList;

#[derive(Clone, Debug)]
pub struct TestCase
{
    pub description: String,
    pub list: LinkedList<i32>,
    pub expected: bool,
}

#[derive(Clone, Deserialize, Debug)]
pub struct TestCaseDeserialized
{
    pub description: String,
    pub list: Vec<i32>,
    pub expected: bool,
}

impl From<&TestCaseDeserialized> for TestCase
{
    fn from(value: &TestCaseDeserialized) -> Self
    {
        let mut list = LinkedList::new();
        for item in &value.list
        {
            list.push_back(*item);
        }
        Self { description: value.description.clone(),
               list,
               expected: value.expected }
    }
}

pub fn read_test_cases() -> Vec<TestCase>
{
    let data_path = Path::new("../TestData/data.json").canonicalize().expect("Invalid path: '../TestData/data.json'");
    let open_error_msg = format!("error opening file: {}", data_path.display());
    let parse_error_msg = format!("error parsing file: {}", data_path.display());
    let file = File::open(data_path).expect(&open_error_msg);
    let test_cases: Vec<TestCaseDeserialized> = serde_json::from_reader(file).expect(&parse_error_msg);
    test_cases.iter().map(|case| case.into()).collect()
}

#[cfg(test)]
type IsPalindromeFn = fn(&LinkedList<i32>) -> bool;

#[cfg(test)]
const IMPLEMENTATIONS: &[(&str, IsPalindromeFn)] = &[("is_palindrome_reverse_and_compare", crate::is_palindrome_reverse_and_compare),
                                                     ("is_palindrome_to_vec_reverse_and_compare", crate::is_palindrome_to_vec_reverse_and_compare),
                                                     ("is_palindrome_deque", crate::is_palindrome_deque),
                                                     ("is_palindrome_two_pointer", crate::is_palindrome_two_pointer),
                                                     ("is_palindrome_stack_half", crate::is_palindrome_stack_half),
                                                     ("is_palindrome_reverse_half", crate::is_palindrome_reverse_half)];

#[test]
fn test_all_implementations()
{
    let test_cases = read_test_cases();
    for (name, is_palindrome) in IMPLEMENTATIONS
    {
        for case in &test_cases
        {
            let result = is_palindrome(&case.list);
            assert_eq!(result, case.expected,
                       "{name} failed for \"{}\": input {:?}, expected is_palindrome = {}, got {}",
                       case.description, case.list, case.expected, result);
        }
    }
}
