# algo-rs

Small Rust workspace for practicing algorithms and data structures.

**Run examples**

```bash
cargo run -- bsearch 9
cargo run -- three_sum
```

Add new algos as single files in `src/algos/` with a `pub fn run(args: &[String]) -> Result<(), String>`.

**How It Works (Auto-Discovery)**

This repo uses a Cargo build script (`build.rs`). If a crate has a `build.rs` at the repo root, Cargo will compile and run it before building the crate.

`build.rs` scans `src/algos/*.rs` and generates a small registry/dispatcher (a Rust source file) into Cargo's build output directory (`OUT_DIR`). `src/algos/mod.rs` then `include!`s that generated file, which provides:

- module declarations for each algo file
- `algos::list()` to show available algos
- `algos::dispatch(name, args)` to run the selected algo

The generated registry is not checked into git; it lives under `target/` (for example: `target/debug/build/<crate>/out/algos_registry.rs`).
