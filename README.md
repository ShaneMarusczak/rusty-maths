# rusty-maths

[![Rust](https://github.com/ShaneMarusczak/rusty-maths/actions/workflows/rust.yml/badge.svg?branch=main&event=push)](https://github.com/ShaneMarusczak/rusty-maths/actions/workflows/rust.yml)

*rusty-maths* is a Rust library and playground for implementing mathematical concepts, covering topics like statistical analysis, linear algebra, geometry, and optimization.

## Quick Start

### Prerequisites
- **Rust** (edition 2021)
- **Cargo** package manager

### Installation
1. Clone the repo:
   `git clone https://github.com/ShaneMarusczak/rusty-maths.git`
2. Enter the directory:
   `cd rusty-maths`
3. Build the project:
   `cargo build`
4. (Optional) Run tests:
   `cargo test`

### Usage
- **Equation Analysis:** Tokenize, parse, and evaluate mathematical expressions.
- **Geometry:** Work with circle operations like calculating area and circumference.
- **Linear Algebra:** Perform vector and matrix operations.
- **Gradient Descent:** Tools for optimization, including gradient estimation.

## Key Features

- **Equation Analysis:** Calculate expressions and plot functions.
- **Geometry:** Operate on circles and more.
- **Linear Algebra:** Vector/matrix operations.
- **Gradient Descent:** Optimization tools.

## Structure

- **Cargo.toml:** Project setup and dependencies (`num_cpus`, `rand`).
- **src/lib.rs:** Library entry-point re-exporting modules.

## Contributing

Contribute by forking, branching, and submitting pull requests with tests and documentation. Open an issue for major changes.

## License

Refer to the [LICENSE](LICENSE) file for licensing details.
