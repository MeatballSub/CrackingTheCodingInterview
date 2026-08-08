//! Three in One: Describe how you could use a single array to implement three
//! stacks.

pub mod fixed_multi_stack;
pub mod flexible_multi_stack;
pub mod test;

#[derive(Debug, PartialEq, Eq)]
pub enum StackError
{
    Full,
    InvalidStack,
}

pub trait ThreeStacks
{
    fn push(&mut self, stack_num: usize, value: i32) -> Result<(), StackError>;

    fn pop(&mut self, stack_num: usize) -> Option<i32>;

    fn peek(&self, stack_num: usize) -> Option<i32>;

    fn is_empty(&self, stack_num: usize) -> bool;
}
