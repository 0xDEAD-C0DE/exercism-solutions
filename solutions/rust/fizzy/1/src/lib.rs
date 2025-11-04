// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

use std::cmp::PartialEq;
/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
use std::ops::Rem;
pub struct Matcher<T> {
    subs: String,
    matcher: fn(T) -> bool,
}

impl<T> Matcher<T> {
    pub fn new<S>(matcher: fn(T) -> bool, subs: S) -> Matcher<T>
    where
        S: ToString,
    {
        Self {
            subs: subs.to_string(),
            matcher,
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
}

impl<T: Copy + Clone + ToString> Fizzy<T> {
    pub fn new() -> Self {
        Self {
            matchers: Vec::new(),
        }
    }

    // feel free to change the signature to `mut self` if you like
    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = T>,
    {
        iter.map(move |e| {
            let mut result: String = self
                .matchers
                .iter()
                .filter(|m| (m.matcher)(e))
                .map(|m| m.subs.clone())
                .collect();

            if result.is_empty() {
                result.push_str(&e.to_string())
            }
            result
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T: ToString + Copy + Rem<Output = T> + PartialEq + From<u8>>() -> Fizzy<T> {
    Fizzy::new()
        .add_matcher(Matcher::new(|n| n % T::from(3) == T::from(0), "fizz"))
        .add_matcher(Matcher::new(|n| n % T::from(5) == T::from(0), "buzz"))
}
