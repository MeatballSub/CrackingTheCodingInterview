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

// Three in One: Describe how you could use a single array to implement three
// stacks.
pub struct FixedMultiStack
{
    stack_capacity: usize,
    values: Vec<i32>,
    sizes: [usize; 3],
}

impl FixedMultiStack
{
    pub fn new(stack_capacity: usize) -> Self { todo!() }
}

impl ThreeStacks for FixedMultiStack
{
    fn push(&mut self, stack_num: usize, value: i32) -> Result<(), StackError> { todo!() }

    fn pop(&mut self, stack_num: usize) -> Option<i32> { todo!() }

    fn peek(&self, stack_num: usize) -> Option<i32> { todo!() }

    fn is_empty(&self, stack_num: usize) -> bool { todo!() }
}
