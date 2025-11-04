/// Yields each item of a and then each item of b

pub fn append<I, J>(mut a: I, mut b: J) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    std::iter::from_fn(move || a.next().or_else(|| b.next()))
}

/// Combines all items in all nested iterators inside into one flattened iterator
pub fn concat<I>(nested_iter: I) -> impl Iterator<Item = <I::Item as Iterator>::Item>
where
    I: Iterator,
    I::Item: Iterator,
{
    let mut l = vec![];
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    for item in nested_iter {
        for nested in item {
            l.push(nested)
        }
    }
    l.into_iter()
}

/// Returns an iterator of all items in iter for which `predicate(item)` is true
pub fn filter<I, F>(mut iter: I, predicate: F) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    // std::iter::from_fn(|| todo!())
    std::iter::from_fn(move || iter.find(|val| (predicate)(&val)))
}

pub fn length<I: Iterator>(mut iter: I) -> usize {
    let mut len = 0;
    loop {
        if iter.next().is_none() {
            break;
        }
        len += 1;
    }

    len

    // todo!("return the total number of items within iter")
}

/// Returns an iterator of the results of applying `function(item)` on all iter items
pub fn map<I, F, U>(mut iter: I, function: F) -> impl Iterator<Item = U>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    std::iter::from_fn(move || match iter.next() {
        Some(v) => Some(function(v)),
        None => None,
    })
}

pub fn foldl<I, F, U>(iter: I, initial: U, function: F) -> U
where
    I: Iterator,
    F: Fn(U, I::Item) -> U,
{
    // todo!("starting with initial, fold (reduce) each iter item into the accumulator from the left")
    let mut accumulator = initial;
    for e in iter {
        accumulator = (function)(accumulator, e);
    }
    accumulator
}

pub fn foldr<I, F, U>(iter: I, initial: U, function: F) -> U
where
    I: DoubleEndedIterator,
    F: Fn(U, I::Item) -> U,
{
    foldl(reverse(iter), initial, function)
    // todo!("starting with initial, fold (reduce) each iter item into the accumulator from the right")
}

/// Returns an iterator with all the original items, but in reverse order
pub fn reverse<I: DoubleEndedIterator>(mut iter: I) -> impl Iterator<Item = I::Item> {
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    std::iter::from_fn(move || iter.next_back())
}
