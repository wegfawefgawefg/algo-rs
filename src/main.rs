fn main() {
    // Usage:
    //   cargo run -- <algo> [args...]
    // Example:
    //   cargo run -- bsearch 9
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("usage: cargo run -- <algo> [args...]");
        eprintln!();
        eprintln!("available algos:");
        for name in algo_rs::algos::list() {
            eprintln!("  {}", name);
        }
        std::process::exit(2);
    }

    let algo = args.remove(0);
    if let Err(err) = algo_rs::algos::dispatch(&algo, &args) {
        eprintln!("{}", err);
        eprintln!();
        eprintln!("available algos:");
        for name in algo_rs::algos::list() {
            eprintln!("  {}", name);
        }
        std::process::exit(2);
    }
}
