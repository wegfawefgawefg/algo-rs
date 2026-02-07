use algo_rs::algos::search::binary_search;

fn main() {
    // Usage:
    //   cargo run --bin bsearch -- 9
    //   cargo run --bin bsearch -- 2
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: bsearch <i32>");
        std::process::exit(2);
    }

    let needle: i32 = args[1].parse().expect("needle must be an i32");
    let haystack = vec![-10, -3, 0, 1, 4, 9, 12, 42];

    match binary_search(&haystack, &needle) {
        Some(i) => println!("found {} at index {} in {:?}", needle, i, haystack),
        None => println!("{} not found in {:?}", needle, haystack),
    }
}
