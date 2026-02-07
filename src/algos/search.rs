/// Returns the index of `needle` in `haystack` (which must be sorted ascending),
/// or `None` if not found.
pub fn binary_search<T: Ord>(haystack: &[T], needle: &T) -> Option<usize> {
    let mut lo = 0usize;
    let mut hi = haystack.len(); // exclusive

    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        match haystack[mid].cmp(needle) {
            std::cmp::Ordering::Less => lo = mid + 1,
            std::cmp::Ordering::Greater => hi = mid,
            std::cmp::Ordering::Equal => return Some(mid),
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_existing() {
        let v = vec![1, 3, 4, 9, 10];
        assert_eq!(binary_search(&v, &1), Some(0));
        assert_eq!(binary_search(&v, &4), Some(2));
        assert_eq!(binary_search(&v, &10), Some(4));
    }

    #[test]
    fn misses_non_existing() {
        let v = vec![1, 3, 4, 9, 10];
        assert_eq!(binary_search(&v, &2), None);
        assert_eq!(binary_search(&v, &11), None);
        assert_eq!(binary_search(&v, &0), None);
    }
}
