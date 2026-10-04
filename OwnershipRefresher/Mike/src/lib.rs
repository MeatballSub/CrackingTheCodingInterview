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

#[must_use]
pub fn contains_word(text: String, word: String) -> bool {
    text.clone()
        .split_whitespace()
        .any(|w| w.to_string() == word.clone())
}

#[must_use]
pub fn total_len(words: Vec<String>) -> usize {
    let mut total = 0;
    for w in words.clone() {
        total += w.len();
    }
    total
}

#[must_use]
pub fn first_capitalized(names: &Vec<String>) -> Option<String> {
    names
        .iter()
        .cloned()
        .find(|n| n.starts_with(char::is_uppercase))
}

#[must_use]
pub fn sum_evens(values: &Vec<i32>) -> i32 {
    values.iter().cloned().filter(|x| x % 2 == 0).sum()
}

pub fn swap_names(a: &mut String, b: &mut String) {
    let tmp = a.clone();
    *a = b.clone();
    *b = tmp;
}

pub fn replace_label(labels: &mut Vec<String>, index: usize, new_label: String) -> String {
    let old = labels[index].clone();
    labels[index] = new_label;
    old
}

#[derive(Default)]
pub struct Inbox {
    messages: Vec<String>,
}

impl Inbox {
    // Scaffolding, not part of the exercise: this one is already fine.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    // Scaffolding, not part of the exercise: this one is already fine.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
    // Judgment call: this works as is, and so do some changes to it. If you change
    // it, say why in your comment.
    pub fn push(&mut self, message: &str) {
        self.messages.push(message.to_string());
    }
    // Scaffolding, not part of the exercise: this one is already fine.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.messages.len()
    }
    pub fn take_all(&mut self) -> Vec<String> {
        let all = self.messages.clone();
        self.messages.clear();
        all
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains() {
        assert!(contains_word(
            "the quick fox".to_string(),
            "quick".to_string()
        ));
        assert!(!contains_word(
            "the quick fox".to_string(),
            "qui".to_string()
        ));
    }

    #[test]
    fn total() {
        let words = vec!["ab".to_string(), "cde".to_string()];
        assert_eq!(total_len(words.clone()), 5);
    }

    #[test]
    fn first() {
        let names = vec!["bob".to_string(), "Alice".to_string()];
        assert_eq!(first_capitalized(&names).as_deref(), Some("Alice"));
    }

    #[test]
    fn sum() {
        assert_eq!(sum_evens(&vec![1, 2, 3, 4]), 6);
    }

    #[test]
    fn swap() {
        let mut a = "a".to_string();
        let mut b = "b".to_string();
        swap_names(&mut a, &mut b);
        assert_eq!((a.as_str(), b.as_str()), ("b", "a"));
    }

    #[test]
    fn replace() {
        let mut labels = vec!["x".to_string()];
        assert_eq!(replace_label(&mut labels, 0, "y".to_string()), "x");
        assert_eq!(labels, ["y"]);
    }

    #[test]
    fn inbox() {
        let mut inbox = Inbox::new();
        inbox.push("hi");
        assert_eq!(inbox.take_all(), ["hi"]);
        assert!(inbox.is_empty());
    }
}
