//! See https://www.codewars.com/kata/585a033e3a36cdc50a00011c/rust

use std::cell::Cell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Write};
use std::rc::Rc;

/// Zero-cost abstraction for shared counters
#[derive(Clone,Default)]
struct Counter {
    count: Rc<Cell<usize>>,
}

impl Counter {
    /// Get current value
    fn get(&self) -> usize {
        self.count.get()
    }
    /// Increment value
    fn inc(&mut self) -> usize {
        let previous = self.count.get();
        let next = previous + 1;
        self.count.set(next);
        next
    }
}

impl Display for Counter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}

/// Replace each character by this frequency, separating frequencies by given separator
///
/// How to:
/// * Build an index of shared counter for each character
/// * Map each character to its shared counters
/// * Finally, just join frequency sequence by given separator
///
/// Example:
/// * Input: `Hello`
///   * Index: `{:}`, Frequences: `[]`
///   * Index: `{H:1}`, Frequences: `[1]`
///   * Index: `{H:1, e:1}`, Frequences: `[1, 1]`
///   * Index: `{H:1, e:1, l:1}`, Frequences: `[1, 1, 1]`
///   * Index: `{H:1, e:1, l:2}`, Frequences: `[1, 1, 2, 2]`
///   * Index: `{H:1, e:1, l:2, o:1}`, Frequences: `[1, 1, 2, 2, 1]`
///
/// _Note: waiting [`std::slice::Join` trait](https://doc.rust-lang.org/std/slice/trait.Join.html#) to
/// become stable._
pub fn freq_seq(input: &str, sep: &str) -> String {
    let mut index: HashMap<_, Counter> = HashMap::new();
    let frequences: Vec<_> = input
        .chars()
        .map(|char| {
            let value = index.entry(char).or_default();
            value.inc();
            value.clone()
        })
        .collect();

    // May be replaced by [`std::slice::Join` trait](https://doc.rust-lang.org/std/slice/trait.Join.html#)
    let mut result = String::new();
    for counter in frequences {
        if result.is_empty() {
            write!(&mut result, "{}", counter)
        } else {
            write!(&mut result, "{}{}", sep, counter)
        }.unwrap();
    }
    result
}

#[cfg(test)]
mod test {
    use super::freq_seq;

    #[test]
    fn hello_dash() {
        assert_eq!(
            freq_seq("hello", "-"),
            "1-1-2-2-1",
        )
    }

    #[test]
    fn returns_expected() {
        assert_eq!(freq_seq("hello world", "-"), "1-1-3-3-2-1-1-2-1-3-1");
        assert_eq!(freq_seq("19999999", ":"), "1:7:7:7:7:7:7:7");
        assert_eq!(freq_seq("^^^**$", "x"), "3x3x3x2x2x1");
    }
}
