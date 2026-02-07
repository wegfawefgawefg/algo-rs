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

pub fn run(args: &[String]) -> Result<(), String> {
    // Example:
    //   cargo run -- bsearch 9
    if args.len() != 1 {
        return Err("usage: bsearch <i32>".to_string());
    }

    let needle: i32 = args[0]
        .parse()
        .map_err(|_| "needle must be an i32".to_string())?;
    let haystack = vec![-10, -3, 0, 1, 4, 9, 12, 42];

    match binary_search(&haystack, &needle) {
        Some(i) => println!("found {} at index {} in {:?}", needle, i, haystack),
        None => println!("{} not found in {:?}", needle, haystack),
    }

    Ok(())
}
