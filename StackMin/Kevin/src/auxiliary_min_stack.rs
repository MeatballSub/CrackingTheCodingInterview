use crate::MinStack;

/// A plain value stack alongside a *sparse* stack of minimums: `mins` gains an
/// entry only when a pushed value ties or beats the current minimum, and loses
/// one only when that value is popped. So `mins` is at most as long as `values`
/// (length 1 for a wholly ascending run), where [`PairedMinStack`] always
/// stores one minimum per element.
///
/// Invariant: `mins` is empty exactly when `values` is, and `mins.last()` is
/// the minimum of `values`. Ties push a duplicate onto `mins` so that popping
/// one of several equal minimums leaves the rest intact.
///
/// [`PairedMinStack`]: crate::paired_min_stack::PairedMinStack
#[derive(Debug, Default)]
pub struct AuxiliaryMinStack
{
    values: Vec<i32>,
    mins: Vec<i32>,
}

impl AuxiliaryMinStack
{
    #[must_use]
    pub fn new() -> Self { Self::default() }
}

impl MinStack for AuxiliaryMinStack
{
    fn push(&mut self, value: i32)
    {
        let is_new_min = self.min().is_none_or(|old_min| value <= old_min);
        self.values.push(value);
        if is_new_min
        {
            self.mins.push(value);
        }
    }

    fn pop(&mut self) -> Option<i32>
    {
        let value = self.values.pop()?;
        if self.min() == Some(value)
        {
            self.mins.pop();
        }
        Some(value)
    }

    fn min(&self) -> Option<i32> { self.mins.last().copied() }

    fn is_empty(&self) -> bool { self.values.is_empty() }
}
