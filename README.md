# Advent of Code 2024 - Rust Solutions

This repository contains my solutions for [Advent of Code 2024](https://adventofcode.com/2024) implemented in Rust.

## Project Structure

```
advent-of-code-2024/
├── Cargo.toml
├── README.md
├── .gitignore
├── input/
│   └── dayXX.txt
└── src/
    ├── main.rs
    ├── solutions/
    │   ├── mod.rs
    │   └── dayXX.rs
    └── utils/
        ├── mod.rs
        └── input.rs
```

## Usage

To run a specific day's solution:

```bash
cargo run --release -- <day>
```

For example, to run day 1:
```bash
cargo run --release -- 1
```

## Progress

- [ ] Day 1
- [ ] Day 2
- [ ] Day 3
...
- [ ] Day 25

## Implementation Details

Each day's solution is implemented in a separate module under `src/solutions/`. The `utils` directory contains helper functions for common operations like reading input files.

## Dependencies

See `Cargo.toml` for the complete list of dependencies.

## License

MIT License