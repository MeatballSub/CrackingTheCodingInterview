// Every function below works, and all seven tests pass, but most of them
// clone, allocate or take ownership when they don't need to. Find each one
// that does and fix it. A fix can be any mix of:
//   - changing a parameter or return type, for example borrowing instead of
//     taking ownership,
//   - rewriting the body, up to replacing it entirely,
//   - updating the tests that call it to match.
// Above each function you change, write a short comment explaining why
// your version compiles without the clone. Finish with all tests passing,
// no compiler warnings, and default cargo clippy clean.

// Only reading the parameters, no need to take ownership, no need to
// allocate/convert with to_string for comparison
#[must_use]
pub fn contains_word(text: &str, word: &str) -> bool { text.split_whitespace().any(|w| w == word) }

// Only reading the parameter, no need to take ownership of the Vec, the loop
// will now also be over references to the individual words no need to copy them
#[must_use]
pub fn total_len(words: &[String]) -> usize
{
    let mut total = 0;
    for w in words
    {
        total += w.len();
    }
    total
}

// Only reading the parameter, no need to take ownership, can return a reference
// to data in the input parameters
#[must_use]
pub fn first_capitalized(names: &[String]) -> Option<&str> { names.iter().find(|n| n.starts_with(char::is_uppercase)).map(String::as_str) }

// Using a slice for parameter, because it accepts more parameter types than a
// Vec does.
// i32 is Copy so `.cloned()` isn't really the intent `.copied()` spells
// out the intent more clearly and gives us the protection that if the element
// type changes from i32 to a non-copy type we don't end up doing a potentially
// expensive clone instead
#[must_use]
pub fn sum_evens(values: &[i32]) -> i32 { values.iter().copied().filter(|x| x % 2 == 0).sum() }

// Can't move out of &mut, you have to replace it with something
// std::mem::swap does that for both items at the same time with no need to
// clone or create an extra temporary variable
pub const fn swap_names(a: &mut String, b: &mut String) { std::mem::swap(a, b); }

// Can't move out of &mut, you have to replace it with something
// std::mem::replace can put new_label in without cloning the old value first
// We can use a slice instead of a Vec because we're not changing anything about
// the size of the Vec just modifying an element
pub fn replace_label(labels: &mut [String], index: usize, new_label: String) -> String { std::mem::replace(&mut labels[index], new_label) }

#[derive(Default)]
pub struct Inbox
{
    messages: Vec<String>,
}

impl Inbox
{
    // Scaffolding, not part of the exercise: this one is already fine.
    #[must_use]
    pub fn new() -> Self { Self::default() }

    // Scaffolding, not part of the exercise: this one is already fine.
    #[must_use]
    pub const fn is_empty(&self) -> bool { self.messages.is_empty() }

    // Judgment call: this works as is, and so do some changes to it. If you change
    // it, say why in your comment.
    // By taking a String instead of &str, we never copy the message, and move the
    // choice of allocation or giving up ownership to the caller
    pub fn push(&mut self, message: String) { self.messages.push(message); }

    // Scaffolding, not part of the exercise: this one is already fine.
    #[must_use]
    pub const fn len(&self) -> usize { self.messages.len() }

    // Can't move out of &mut, you have to replace it with something
    // std::mem::take will replace it with Vec::default() an empty Vec
    pub fn take_all(&mut self) -> Vec<String> { std::mem::take(&mut self.messages) }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn contains()
    {
        assert!(contains_word("the quick fox", "quick"));
        assert!(!contains_word("the quick fox", "qui"));
    }

    #[test]
    fn total()
    {
        let words = vec!["ab".to_string(), "cde".to_string()];
        assert_eq!(total_len(&words), 5);
    }

    #[test]
    fn first()
    {
        let names = vec!["bob".to_string(), "Alice".to_string()];
        assert_eq!(first_capitalized(&names), Some("Alice"));
    }

    #[test]
    fn sum()
    {
        assert_eq!(sum_evens(&[1, 2, 3, 4]), 6);
    }

    #[test]
    fn swap()
    {
        let mut a = "a".to_string();
        let mut b = "b".to_string();
        swap_names(&mut a, &mut b);
        assert_eq!((a.as_str(), b.as_str()), ("b", "a"));
    }

    #[test]
    fn replace()
    {
        let mut labels = vec!["x".to_string()];
        assert_eq!(replace_label(&mut labels, 0, "y".to_string()), "x");
        assert_eq!(labels, ["y"]);
    }

    #[test]
    fn inbox()
    {
        let mut inbox = Inbox::new();
        inbox.push("hi".to_string());
        assert_eq!(inbox.take_all(), ["hi"]);
        assert!(inbox.is_empty());
    }
}
