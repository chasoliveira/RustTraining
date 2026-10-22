# RustTraining

This repository contains my exercises and projects while learning Rust.

I come from a C# background, so I am following Microsoft's [Rust for C# Programmers training guide](https://microsoft.github.io/RustTraining/csharp-book/). The guide compares Rust concepts with familiar C# and .NET concepts, making it a practical path for learning the language and its different approach to memory safety, ownership, error handling, and concurrency.

My goal is to complete all of the exercises proposed throughout the guide and keep the solutions here as a record of my progress.

## Learning path

The training covers:

- Rust fundamentals, types, variables, and control flow
- Structs, collections, enums, and pattern matching
- Ownership, borrowing, references, and lifetimes
- Modules, crates, and Cargo
- Error handling with `Result` and the `?` operator
- Traits, generics, conversions, closures, and iterators
- Concurrency and asynchronous Rust
- Unsafe Rust, FFI, testing, and C# interoperability
- Migration patterns and idiomatic Rust practices
- A capstone CLI weather application

## Repository structure

Each directory is intended to contain an exercise or a small Rust project. Currently, the repository includes:

```text
type_safe_temperature/
├── Cargo.toml
└── src/
    └── main.rs
```

### Current exercises

- `type_safe_temperature` — a temperature-conversion exercise using typed parsing, constants, validation, and an atomic conversion counter.

## Running an exercise

From an exercise directory, run:

```bash
cargo run
```

To check formatting, compile the project, and run tests:

```bash
cargo fmt --check
cargo check
cargo test
```

## Progress

Exercises will be added as I work through the training guide. Solutions are written for learning purposes, with an emphasis on understanding Rust’s concepts rather than simply translating C# syntax directly.
