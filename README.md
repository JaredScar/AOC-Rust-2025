# Advent of Code 2025 - Rust Solutions

This repository contains solutions to the [Advent of Code 2025](https://adventofcode.com/2025) challenges implemented in Rust.

## Prerequisites

### 1. Install Rust

Install Rust using [rustup](https://rustup.rs/), the official Rust installer:

**Windows:**
```bash
# Download and run rustup-init.exe from https://rustup.rs/
# Or use PowerShell:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**macOS/Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Install Visual Studio Build Tools (Windows Only)

If you're on Windows, you'll need Visual Studio Build Tools for linking. The error you might see:

```
error: linking with link.exe failed
note: you may need to install Visual Studio build tools with the "C++ build tools" workload
```

**Solution:**

1. Download [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022)
2. During installation, select the **"C++ build tools"** workload
3. Make sure "Windows 10/11 SDK" is included
4. Restart your terminal/IDE after installation

Alternatively, you can install [Visual Studio Community](https://visualstudio.microsoft.com/vs/community/) with the "Desktop development with C++" workload.

**Note:** On macOS/Linux, you typically don't need additional build tools beyond what comes with the system.

## Getting Started

### Clone and Build

```bash
# Clone the repository (if applicable)
git clone <your-repo-url>
cd AOC-Rust-2025

# Build the project
cargo build

# Run the project
cargo run
```

### Project Structure

```
AOC-Rust-2025/
├── Cargo.toml          # Project configuration and dependencies
├── src/
│   ├── main.rs         # Main entry point - uncomment the day you want to run
│   ├── day01.rs        # Day 1 solution
│   ├── day02.rs        # Day 2 solution
│   └── ...             # Additional day solutions
└── inputs/
    ├── day01.txt       # Day 1 input file
    ├── day02.txt       # Day 2 input file
    └── ...             # Additional input files
```

### Running Specific Days

Edit `src/main.rs` to uncomment the day you want to run:

```rust
mod day01;
mod day02;

fn main() {
    day01::run();  // Uncomment to run day 1
    // day02::run();  // Comment out other days
}
```

Then run:
```bash
cargo run
```

## Rust Tips for Advent of Code

### Switch/Case Statements

Rust uses `match` instead of `switch`:

```rust
// Instead of switch/case:
match value {
    1 => println!("One"),
    2 => println!("Two"),
    3 => println!("Three"),
    _ => println!("Other"),  // Default case
}

// Match with ranges:
match number {
    0..=9 => println!("Single digit"),
    10..=99 => println!("Double digit"),
    _ => println!("Large number"),
}
```

### Ternary Operator

Rust doesn't have a ternary operator (`condition ? true_value : false_value`). Use `if-else` expressions instead:

```rust
// Instead of: let result = condition ? value1 : value2;
let result = if condition {
    value1
} else {
    value2
};

// Can be used inline:
let max = if a > b { a } else { b };
```

### Common Patterns

**Reading files:**
```rust
use std::fs;

let input = fs::read_to_string("inputs/day01.txt")
    .expect("Failed to read input file");
```

**Parsing numbers:**
```rust
let num: i32 = "123".parse().unwrap();
// Or with error handling:
let num: i32 = "123".parse().expect("Not a number");
```

**Iterating over lines:**
```rust
for line in input.lines() {
    // Process each line
}
```

## Troubleshooting

### Linking Errors on Windows

If you see linking errors, ensure Visual Studio Build Tools are installed (see Prerequisites section above).

### Input File Not Found

Make sure your input files are in the `inputs/` directory relative to where you run `cargo run`. The working directory should be the project root.

### Common Rust Errors

- **`unwrap()` panics**: Use `expect()` with a message or proper error handling
- **Borrow checker errors**: Rust's ownership system - read the error messages carefully
- **Type mismatches**: Rust is strongly typed - check your type annotations

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Advent of Code 2025](https://adventofcode.com/2025)
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/)

## License

This project is for personal use in solving Advent of Code challenges.

