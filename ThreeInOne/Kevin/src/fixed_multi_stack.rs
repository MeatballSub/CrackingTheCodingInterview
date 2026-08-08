use crate::StackError;
use crate::ThreeStacks;

const NUM_STACKS: usize = 3;

#[derive(Debug)]
pub struct FixedMultiStack
{
    stack_capacity: usize,
    values: Box<[i32]>,
    sizes: [usize; NUM_STACKS],
}

impl FixedMultiStack
{
    pub fn new(stack_capacity: usize) -> Self
    {
        Self { stack_capacity,
               values: vec![0; stack_capacity * NUM_STACKS].into_boxed_slice(),
               sizes: [0; NUM_STACKS] }
    }
}

impl ThreeStacks for FixedMultiStack
{
    fn push(&mut self, stack_num: usize, value: i32) -> Result<(), StackError>
    {
        if stack_num >= NUM_STACKS
        {
            return Err(StackError::InvalidStack);
        }

        if self.sizes[stack_num] >= self.stack_capacity
        {
            return Err(StackError::Full);
        }

        let index = (stack_num * self.stack_capacity) + self.sizes[stack_num];
        self.values[index] = value;
        self.sizes[stack_num] += 1;
        Ok(())
    }

    fn pop(&mut self, stack_num: usize) -> Option<i32>
    {
        let value = self.peek(stack_num)?;
        self.sizes[stack_num] -= 1;
        Some(value)
    }

    fn peek(&self, stack_num: usize) -> Option<i32>
    {
        if self.is_empty(stack_num)
        {
            return None;
        }
        let top = (stack_num * self.stack_capacity) + self.sizes[stack_num] - 1;
        Some(self.values[top])
    }

    fn is_empty(&self, stack_num: usize) -> bool { stack_num >= NUM_STACKS || self.sizes[stack_num] == 0 }
}
