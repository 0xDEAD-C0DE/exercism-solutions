pub fn find<A: AsRef<[T]>, T>(array: A, key: T) -> Option<usize>
where
    T: PartialEq + PartialOrd,
{
    if array.as_ref().is_empty() {
        return None;
    }
    let mut left: usize = 0;
    let mut right = array.as_ref().len() - 1;
    let mut found = false;
    let mut middle = 0;

    while left <= right {
        middle = (left + right) / 2;

        if array.as_ref().get(middle).unwrap() < &key {
            left = middle + 1;
        } else if array.as_ref().get(middle).unwrap() > &key {
            right = match middle.checked_sub(1) {
                Some(n) => n,
                _ => break,
            };
        } else {
            found = true;
            break;
        }
    }
    if left > right {
        return None;
    }
    if found {
        Some(middle)
    } else {
        None
    }
}
