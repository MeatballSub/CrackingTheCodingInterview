# Ownership Refresher - Quick notes

The one idea for this lesson: **every value has exactly one owner, and `.clone()` is rarely the right way to satisfy the compiler.** We need to get this absolutely solid.

## Goal and outcomes

Everyone should be able to:

1. Say whether a line of code **moves, copies or clones** a value, and why.
2. Pick `&str` or `String` (and `&[T]` or `Vec<T>`) for a parameter, and explain the choice.
3. Say what `for x in v`, `for x in &v` and `for x in &mut v` each give you, and what's left of `v` afterward.
4. Move a value out from behind a `&mut` with `mem::take`, `mem::replace` or `mem::swap`, without cloning.
5. Read E0382 and E0507 and fix them without reaching for `.clone()`.

The last outcome matters most: moving a value out from behind a `&mut` (E0507) comes up constantly in linked-lists and trees.

## Initial Reading

[*The Rust Programming Language*, chapter 4 ("Understanding Ownership").](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
Everything beyond here is somewhat summarized given you've done the pre-requisite reading.

## Highlights
Your new mantra: "If you can't say why a `.clone()` is needed, it probably isn't."

### Move, copy, clone

```rust
let a: i32 = 5;
let b = a;          // copy: `a` is still usable

let s = String::from("Ferris");
let t = s;          // move: `s` is gone, `t` owns the heap buffer
let u = t.clone();  // clone: an explicit second buffer; `t` still usable
```

- **Copy** is an implicit bit-for-bit copy. Only simple types with no heap data and no `Drop` can be `Copy`, such as integers, `char`, `bool` and `&T`.
- **Move** is the default for everything else. The bits are copied, but the old name can't be used any more.
- **Clone** is explicit and may allocate. It's a decision, not a way to make an error go away.

### Borrowing: `String` vs. `&str`

```rust
fn shout(s: &str) -> String { s.to_uppercase() }

let owned = String::from("hi");
shout(&owned);      // &String converts to &str automatically
shout("literal");   // a &'static str works too

let r: &str = "hello";
let r2 = r.clone();         // r2: &str. This copied the *reference*, not the text
let o = r.to_string();      // String: a new heap allocation
assert!(o == r);            // String == &str compares without allocating
```

- **Parameters:** take `&str` / `&[T]` when you only read the data. Take `String` / `Vec<T>` only when the function keeps it.
- **Return values:** return an owned value when you create something new. Return a borrow when you're pointing into the input.
- **Calling `.clone()` on a `&str` is a no-op** in practice: you get another reference to the same text. To get an owned copy, use `to_string()` or `to_owned()`.
- **You can compare without converting:** a `String` can be compared to a `&str` directly, with no `.to_string()` first.

### Three ways to iterate

| You write | Same as | Each item is | Afterward, `v` is |
| --- | --- | --- | --- |
| `for x in v` | `v.into_iter()` | `T` (owned) | gone (moved) |
| `for x in &v` | `v.iter()` | `&T` | unchanged |
| `for x in &mut v` | `v.iter_mut()` | `&mut T` | still yours, possibly modified |

**(E0382: borrow of moved value):**

```rust
let v = vec![String::from("a")];
for s in v { println!("{s}"); }
println!("{}", v.len());   // error[E0382]
```

The compiler notes that *"`v` moved due to this implicit call to `.into_iter()`"* and suggests iterating over a slice, which here means `for s in &v` instead.

```rust
let v = vec![String::from("a")];
for s in &v { println!("{s}"); }
println!("{}", v.len());   // error[E0382]
```

#### If you need owned values out of your iterators that return borrowed ones:

For types that are Copy, use `.copied()` instead of `.cloned()`: it's free and says what you mean.  It also offers some future proofing.  If the type gets changed to a non-copy type, `.cloned()` will continue to work but will potentially be very expensive.  On the other hand `.copied()` would cause a compiler error in that situation and force the person making the change to think about the consequences.

### Moving out from behind a `&mut` with `mem::take`, `mem::replace` or `mem::swap`, without cloning.

**(E0507: cannot move out of `self.books` which is behind a mutable reference):**

```rust
struct Shelf { books: Vec<String> }

impl Shelf {
    fn empty_out(&mut self) -> Vec<String> {
        let out = self.books;   // error[E0507]
        out
    }
}
```

Why it fails: we only *borrowed* the shelf. Moving `books` out would leave the shelf with nothing where `books` should be. The compiler suggests borrowing, but here we really do want ownership. The fix is to **swap something valid in**:

```rust
use std::mem;

mem::take(&mut self.books)                        // leaves Vec::new() behind (needs Default)
mem::replace(&mut self.title, String::from("-"))  // leaves what you pass in
mem::swap(&mut a, &mut b)                         // exchanges two values
```

### FAQ
| Question | Short answer |
| --- | --- |
| A function takes a `Vec<i32>` but only reads it. What does that force every caller to do? | Every caller has to give up their vector, or clone it first if they still need it. Callers holding an array, a slice or part of a vector have to allocate a new Vec just to make the call. Taking `&[i32]` accepts all of those with no allocation. |
| Why is `i32` `Copy` but `String` isn't? | A `String` owns a heap buffer. A bit-for-bit copy would give two owners of one buffer, and both would free it. |
| Is `.clone()` slow? | It depends on the type. It's free for `Copy` types, but a `Vec<String>` clone allocates for every element. The bigger problem is that it hides ownership mistakes. |
| Isn't `&String` the same as `&str`? | They're close, and `&String` converts to `&str` automatically. But `&str` also accepts literals and slices of other strings, so it's the more flexible parameter type. |
| Why does `for x in v` use up `v`? | It calls `into_iter()`, which takes `v` by value so it can hand out owned items. |
| `mem::take` vs. `mem::replace`? | `take` is `replace` with `Default::default()` as the value left behind. |
| When is cloning the right call? | When you really need two independent copies, e.g. keeping the original while changing a copy. It's probably not a bad idea to drop a comment as to why for the next dev to look at your code. |
| Does the compiler optimize the clones away? | Sometimes, but you can't count on it, and the code still reads as if two owners are needed. |