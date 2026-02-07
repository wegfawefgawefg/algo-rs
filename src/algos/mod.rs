// build.rs generates module declarations + a small registry/dispatcher here.
include!(concat!(env!("OUT_DIR"), "/algos_registry.rs"));
