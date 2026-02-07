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
