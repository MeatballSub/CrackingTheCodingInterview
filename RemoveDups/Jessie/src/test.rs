use std::fs::File;
use std::path::Path;

use serde::Deserialize;

use crate::linked_list::LinkedList;

#[derive(Clone, Debug)]
pub struct TestCase
{
    pub list: LinkedList<i32>,
    pub answer: LinkedList<i32>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct TestCaseDeserialized
{
    pub list: Vec<i32>,
    pub answer: Vec<i32>,
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
        let mut answer = LinkedList::new();
        for item in &value.answer
        {
            answer.push_back(*item);
        }
        Self { list, answer }
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
