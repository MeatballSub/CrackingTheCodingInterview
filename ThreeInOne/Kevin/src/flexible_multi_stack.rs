use crate::StackError;
use crate::ThreeStacks;

const NUM_STACKS: usize = 3;

#[derive(Debug)]
struct StackNode
{
    prev: Option<usize>,
    value: i32,
}

#[derive(Debug)]
pub struct FlexibleMultiStack
{
    total_capacity: usize,
    values: Vec<StackNode>,
    free: Vec<usize>,
    top: [Option<usize>; NUM_STACKS],
}

impl FlexibleMultiStack
{
    pub fn new(stack_capacity: usize) -> Self
    {
        let total_capacity = stack_capacity * NUM_STACKS;
        Self { total_capacity,
               values: Vec::with_capacity(total_capacity),
               free: Vec::new(),
               top: [None; NUM_STACKS] }
    }

    fn occupied(&self) -> usize { self.values.len() - self.free.len() }

    fn top_index(&self, stack_num: usize) -> Option<usize>
    {
        if stack_num >= NUM_STACKS
        {
            return None;
        }
        self.top[stack_num]
    }

    fn push_target(&self, stack_num: usize) -> Result<Option<usize>, StackError>
    {
        if stack_num >= NUM_STACKS
        {
            return Err(StackError::InvalidStack);
        }

        if self.occupied() >= self.total_capacity
        {
            return Err(StackError::Full);
        }

        Ok(self.top[stack_num])
    }

    fn claim_slot(&mut self, node: StackNode) -> usize
    {
        match self.free.pop()
        {
            Some(slot) =>
            {
                self.values[slot] = node;
                slot
            }
            None =>
            {
                self.values.push(node);
                self.values.len() - 1
            }
        }
    }

    fn release_slot(&mut self, index: usize) { self.free.push(index); }
}

impl ThreeStacks for FlexibleMultiStack
{
    fn push(&mut self, stack_num: usize, value: i32) -> Result<(), StackError>
    {
        let prev = self.push_target(stack_num)?;

        let index = self.claim_slot(StackNode { prev, value });
        self.top[stack_num] = Some(index);
        Ok(())
    }

    fn pop(&mut self, stack_num: usize) -> Option<i32>
    {
        let top_index = self.top_index(stack_num)?;
        self.top[stack_num] = self.values[top_index].prev;
        self.release_slot(top_index);
        Some(self.values[top_index].value)
    }

    fn peek(&self, stack_num: usize) -> Option<i32> { self.top_index(stack_num).map(|index| self.values[index].value) }

    fn is_empty(&self, stack_num: usize) -> bool { self.top_index(stack_num).is_none() }
}
