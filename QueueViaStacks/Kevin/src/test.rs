use std::fs::read_to_string;
use std::path::Path;

use serde::Deserialize;

use crate::MyQueue;
use crate::NamedQueue;
use crate::eager_two_stack_queue::EagerTwoStackQueue;
use crate::lazy_two_stack_queue::LazyTwoStackQueue;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Operation
{
    Enqueue
    {
        value: i32
    },
    Dequeue
    {
        expected: Option<i32>
    },
    Peek
    {
        expected: Option<i32>
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
    let data_path = Path::new(path).canonicalize().unwrap_or_else(|_| panic!("Invalid path: '{path}'"));
    let open_error_msg = format!("error opening file: {}", data_path.display());
    let parse_error_msg = format!("error parsing file: {}", data_path.display());
    let text = read_to_string(&data_path).expect(&open_error_msg);
    serde_json::from_str(&text).expect(&parse_error_msg)
}

pub fn read_test_cases() -> Vec<TestCase> { read_test_data(SHARED_TEST_DATA) }

const BENCH_TEST_DATA: &str = "../TestData/benchmark.json";

pub fn read_bench_cases() -> Vec<TestCase> { read_test_data(BENCH_TEST_DATA) }

pub fn run_operations<Q: MyQueue + ?Sized>(queue: &mut Q, case: &TestCase, label: &str)
{
    for (i, operation) in case.operations.iter().enumerate()
    {
        match operation
        {
            Operation::Enqueue { value } =>
            {
                queue.enqueue(*value);
            }
            Operation::Dequeue { expected } =>
            {
                assert_eq!(queue.dequeue(), *expected, "{label} / {}: op {i}: dequeue mismatch", case.description);
            }
            Operation::Peek { expected } =>
            {
                assert_eq!(queue.peek(), *expected, "{label} / {}: op {i}: peek mismatch", case.description);
            }
            Operation::IsEmpty { expected } =>
            {
                assert_eq!(queue.is_empty(), *expected, "{label} / {}: op {i}: is_empty mismatch", case.description);
            }
            Operation::Len { expected } =>
            {
                assert_eq!(queue.len(), *expected, "{label} / {}: op {i}: len mismatch", case.description);
            }
        }
    }
}

pub type MyQueueCtor = fn() -> Box<dyn MyQueue>;

fn make<Q: NamedQueue + 'static>() -> Box<dyn MyQueue> { Box::new(Q::default()) }

macro_rules! implementations {
    ($($queue:ty),+ $(,)?) =>
    {
        pub const IMPLEMENTATIONS: &[(&str, MyQueueCtor)] = &[$((<$queue as NamedQueue>::NAME, make::<$queue> as MyQueueCtor)),+];
    };
}

implementations!(EagerTwoStackQueue, LazyTwoStackQueue);

#[test]
fn test_all_implementations()
{
    let shared_cases = read_test_cases();
    for &(name, make) in IMPLEMENTATIONS
    {
        for case in &shared_cases
        {
            let mut queue = make();
            run_operations(&mut *queue, case, name);
        }
    }
}
