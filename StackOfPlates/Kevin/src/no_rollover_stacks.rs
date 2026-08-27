use crate::SetOfStacks;

#[derive(Debug)]
pub struct NoRolloverStacks
{
    capacity: usize,
    stacks: Vec<Vec<i32>>,
}

impl NoRolloverStacks
{
    #[must_use]
    pub fn new(capacity: usize) -> Self { Self { capacity, stacks: Vec::new() } }
}

impl SetOfStacks for NoRolloverStacks
{
    fn push(&mut self, value: i32)
    {
        match self.stacks.last_mut()
        {
            Some(last_stack) if last_stack.len() < self.capacity =>
            {
                last_stack.push(value);
            }
            _ =>
            {
                let mut new_stack = Vec::with_capacity(self.capacity);
                new_stack.push(value);
                self.stacks.push(new_stack);
            }
        }
    }

    fn pop(&mut self) -> Option<i32>
    {
        let last_stack = self.stacks.last_mut()?;
        let popped_item = last_stack.pop();

        if last_stack.is_empty()
        {
            self.stacks.pop();
        }

        popped_item
    }

    fn pop_at(&mut self, index: usize) -> Option<i32>
    {
        let target_stack = self.stacks.get_mut(index)?;
        let popped_item = target_stack.pop();

        if target_stack.is_empty()
        {
            self.stacks.remove(index);
        }

        popped_item
    }

    fn peek(&self) -> Option<i32> { self.stacks.last()?.last().copied() }

    fn is_empty(&self) -> bool { self.stacks.is_empty() }

    fn stack_count(&self) -> usize { self.stacks.len() }
}
