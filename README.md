# grip – A Rusty grep Alternative

[![Crates.io](https://img.shields.io/crates/v/grip.svg)](https://crates.io/crates/grip)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

`grip` is a simple, fast command-line tool for searching text in files, written in Rust. Inspired by `grep`, but built for learning and experimentation.

## Features
✅ **Basic text search** (like `grep`)

✅ **Fast file reading** (thanks to Rust!)

✅ **Easy to extend** (add regex, colors, etc.)

## Installation
### From Crates.io (coming soon)
```sh
cargo install grip
```
### From Source
```sh
git clone https://github.com/skynatbs/grip.git
cd grip
cargo build --release
```
### Usage
```sh
grip <SEARCH-PATTERN> <FILE>
```
### Example
```sh
grip "fn main" src/main.rs
```
### Why `grip`?
- Learning Rust: A great project to understand file I/O, CLI tools, and error handling.
- Performance: Rust makes it fast by default.
- Customizable: Extend it with regex, colors, or parallel processing.

  ### License
  **MIT** (see [LICENSE](https://github.com/skynatbs/grip/blob/main/LICENSE))
