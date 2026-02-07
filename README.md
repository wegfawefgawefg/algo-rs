# algo-rs

Small Rust workspace for practicing algorithms and data structures.

**Run examples**

```bash
cargo run -- bsearch 9
cargo run -- three_sum
```

Add new algos as single files in `src/algos/` with a `pub fn run(args: &[String]) -> Result<(), String>`.
