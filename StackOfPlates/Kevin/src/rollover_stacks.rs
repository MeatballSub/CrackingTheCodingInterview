use crate::SetOfStacks;

#[derive(Debug)]
pub struct RolloverStacks
{
    capacity: usize,
    stacks: Vec<Vec<i32>>,
}

impl RolloverStacks
{
    #[must_use]
    pub fn new(capacity: usize) -> Self { Self { capacity, stacks: Vec::new() } }

    fn shift(&mut self, index: usize)
    {
        for i in index..self.stacks.len().saturating_sub(1)
        {
            let source = self.stacks[i + 1].remove(0);
            self.stacks[i].push(source);
        }

        if self.stacks.last().is_some_and(Vec::is_empty)
        {
            self.stacks.pop();
        }
    }
}

impl SetOfStacks for RolloverStacks
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
        let popped_item = self.stacks.get_mut(index)?.pop()?;

        self.shift(index);

        Some(popped_item)
    }

    fn peek(&self) -> Option<i32> { self.stacks.last()?.last().copied() }

    fn is_empty(&self) -> bool { self.stacks.is_empty() }

    fn stack_count(&self) -> usize { self.stacks.len() }
}
