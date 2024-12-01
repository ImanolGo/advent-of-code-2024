# Advent of Code 2024 - Rust Solutions

This repository contains my solutions for [Advent of Code 2024](https://adventofcode.com/2024) implemented in Rust.

## Setup

1. Make sure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/)
2. Clone this repository:
```bash
git clone <your-repository-url>
cd advent-of-code-2024
```

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

## Adding New Solutions

1. Create your input file for the day:
   ```bash
   # Create input file (replace XX with day number, e.g., 01)
   touch input/dayXX.txt
   ```

2. Copy your puzzle input from Advent of Code into this file

3. Create a new solution file:
   ```bash
   # Create solution file (replace XX with day number)
   cp src/solutions/template.rs src/solutions/dayXX.rs
   ```

4. Update `src/solutions/mod.rs` to include your new day's module

## Running Solutions

To run a specific day's solution:

```bash
# Run with default configuration
cargo run --release -- <day>

# Example for day 1
cargo run --release -- 1
```

To run in debug mode (slower but with more checks):
```bash
cargo run -- <day>
```

## Running Tests

```bash
# Run all tests
cargo test

# Run tests for a specific day
cargo test --package advent-of-code-2024 --test day01

# Run tests with output
cargo test -- --nocapture
```

## Development Tips

1. Each day's solution file (`dayXX.rs`) contains:
   - Main solution functions for both parts
   - Tests using the example input provided in the puzzle
   - Helper functions specific to that day's puzzle

2. The `utils` module contains common functionality used across multiple days

3. Input files are read automatically from the `input` directory

4. Test input can be added directly in the solution files under the `tests` module

## Debugging

1. You can add debug output using:
```rust
println!("Debug: {:?}", variable);
```

2. For more complex debugging, consider using:
```rust
dbg!(variable);
```

3. You can also use the built-in debugger:
```bash
rust-lldb target/debug/advent-of-code-2024 1
```

## Performance

The solutions are automatically compiled with optimizations when using `--release`. If you want to benchmark your solution:

```bash
# Run with timing information
time cargo run --release -- <day>
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

Main dependencies included in this project:
- `itertools`: For advanced iterator operations
- `regex`: For pattern matching and text processing
- `lazy_static`: For compile-time computed static variables
- `rayon`: For parallel processing
- `anyhow`: For error handling

See `Cargo.toml` for the complete list of dependencies.

## Contributing

1. Create a branch for your solution
2. Implement your solution
3. Add tests
4. Create a pull request

## License

MIT License