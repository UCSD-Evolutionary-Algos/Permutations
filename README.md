# Permutations

This is a Rust implementation of a genetic algorithm for maximizing the packing density of patterns within permutations.  It leverages data parallelism via [rayon](https://docs.rs/rayon/latest/rayon), computing fitnesses and performing crossover in parallel on the CPU.

This project is a work in progress.

## Building and Running

To build from source, first install [Rust](https://rustup.rs).  Then, run:

```sh
$ cargo run 10 0,1    # Permutation length = 10, pattern = 0,1
```

The following organism should be printed:

```
Organism {
    fitness: 45,
    vals: [
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        9,
    ],
}
```
