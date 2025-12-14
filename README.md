# rusty-maths

[![Rust](https://github.com/ShaneMarusczak/rusty-maths/actions/workflows/rust.yml/badge.svg?branch=main&event=push)](https://github.com/ShaneMarusczak/rusty-maths/actions/workflows/rust.yml)

A high-performance Rust library for mathematical operations, featuring an advanced equation analyzer with multiple pipeline architectures optimized for different use cases.

## Features

### 🚀 High-Performance Equation Analyzer

Optimized pipeline using streaming tokenizer with buffered parser for efficient mathematical expression evaluation.

```rust
use rusty_maths::equation_analyzer::calculator;

// Evaluate an expression
let result = calculator::calculate("2 + 3 * 4").unwrap();
assert_eq!(result, 14.0);

// Plot a function over a range
let points = calculator::plot("x^2 - 2x + 1", -5.0, 5.0, 0.1).unwrap();
```

**Supported Operations:**
- Arithmetic: `+`, `-`, `*`, `/`, `^`, `%`, `%%` (modulo)
- Functions: `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `abs`, `sqrt`, `ln`, `log_N`
- Statistical: `min`, `max`, `avg`, `med` (median), `mode`, `ch` (choice)
- Constants: `e`, `π`
- Variables: `x` with coefficient support (`2x`, `-3x^2`)

### 📊 Statistics & Analysis

Comprehensive statistical functions:
```rust
use rusty_maths::statistics;

let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
let mean = statistics::mean(&data);
let median = statistics::median(&data);
let std_dev = statistics::standard_deviation(&data);
let corr = statistics::correlation(&data1, &data2);
```

### 🔢 Linear Algebra

Vector and matrix operations:
```rust
use rusty_maths::linear_algebra;

let v1 = vec![1.0, 2.0, 3.0];
let v2 = vec![4.0, 5.0, 6.0];

let dot = linear_algebra::dot_product(&v1, &v2);
let sum = linear_algebra::vec_add(&v1, &v2);
let scaled = linear_algebra::scalar_multiply(&v1, 2.0);
```

### 📐 Geometry

Circle calculations and more:
```rust
use rusty_maths::geometry;

let circle = geometry::Circle::new(5.0);
let area = circle.area();
let circumference = circle.circumference();
```

### 📈 Gradient Descent

Optimization algorithms with batch and mini-batch support:
```rust
use rusty_maths::gradient_descent;

// Linear gradient descent
let weights = gradient_descent::linear_gradient(
    &features, &targets, learning_rate, iterations
);

// Mini-batch gradient descent
let weights = gradient_descent::mini_batch(
    &features, &targets, learning_rate, iterations, batch_size
);
```

## Performance

The equation analyzer has been optimized with a streaming tokenizer architecture:

**Features:**
- Iterator-based tokenizer for lazy token generation
- Efficient memory usage with minimal buffering
- Early termination on parse errors
- Parallel evaluation using Rayon for plotting

**Plot Performance** (1000 points, equation: "x^2 + 2x + 1")
- Parse once, evaluate many times
- Scales efficiently with point count

Run benchmarks yourself:
```bash
cargo bench --bench equation_analyzer
```

## Architecture

### Equation Analyzer Pipeline

```
┌──────────────────┐      ┌─────────┐      ┌───────────┐
│ StreamTokenizer  │─────▶│ Parser  │─────▶│ Evaluator │─────▶ f32
│   (Iterator)     │      │         │      │           │
└──────────────────┘      └─────────┘      └───────────┘
         │                     │                  │
    Token (lazy)           Vec<Token>          Result
    on-demand               (RPN)
```

**Pipeline Characteristics:**
- Iterator-based tokenizer for lazy evaluation
- Shunting Yard parser for infix to RPN conversion
- Stack-based RPN evaluator
- Minimal memory overhead with strategic buffering

See [Pipeline README](src/equation_analyzer/pipeline/README.md) for detailed architecture documentation.

## Installation

Add to your `Cargo.toml`:
```toml
[dependencies]
rusty-maths = { git = "https://github.com/ShaneMarusczak/rusty-maths.git" }
```

Or clone and use locally:
```bash
git clone https://github.com/ShaneMarusczak/rusty-maths.git
cd rusty-maths
cargo build --release
```

## Quick Start

```rust
use rusty_maths::equation_analyzer::calculator;
use rusty_maths::statistics;
use rusty_maths::linear_algebra;

fn main() {
    // Equation analysis
    let result = calculator::calculate("sin(π/2) + cos(0)").unwrap();
    println!("Result: {}", result); // 2.0

    // Statistics
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let mean = statistics::mean(&data);
    println!("Mean: {}", mean); // 3.0

    // Linear algebra
    let v1 = vec![1.0, 2.0, 3.0];
    let v2 = vec![4.0, 5.0, 6.0];
    let dot = linear_algebra::dot_product(&v1, &v2);
    println!("Dot product: {}", dot); // 32.0

    // Plot a function
    let points = calculator::plot("x^2", -5.0, 5.0, 0.5).unwrap();
    for point in points.iter().take(5) {
        println!("({}, {})", point.x, point.y);
    }
}
```

## Testing

Run the comprehensive test suite:
```bash
cargo test
```

**350 tests** covering:
- Equation analyzer tests (parsing, evaluation, plotting)
- Statistics, linear algebra, geometry, and gradient descent
- Edge cases and error handling

## Code Quality

The codebase emphasizes:
- **Clean Architecture**: Single, optimized pipeline implementation
- **Zero-Cost Abstractions**: Generic implementations with zero runtime overhead
- **Comprehensive Testing**: 350 tests covering all functionality
- **Performance**: Benchmarked and optimized with criterion
- **Documentation**: Extensive docs for all public APIs and internal architecture

## Dependencies

- `rand` - Random number generation for statistics and gradient descent
- `rayon` - Data parallelism for efficient plotting
- `criterion` (dev) - Benchmarking framework

## Project Structure

```
rusty-maths/
├── src/
│   ├── equation_analyzer/      # Equation parsing and evaluation
│   │   ├── calculator.rs       # Public API (calculate, plot)
│   │   ├── pipeline/           # Internal implementation
│   │   │   ├── tokenizer.rs    # Streaming tokenizer
│   │   │   ├── parser.rs       # Shunting Yard parser
│   │   │   └── evaluator.rs    # RPN evaluator
│   │   └── structs/            # Token and operator definitions
│   ├── statistics/             # Statistical functions
│   ├── linear_algebra/         # Vector/matrix operations
│   ├── geometry/               # Geometric calculations
│   ├── gradient_descent/       # Optimization algorithms
│   └── utilities/              # Helper functions
├── benches/                    # Performance benchmarks
└── tests/                      # Integration tests
```

## Contributing

Contributions are welcome! Please:
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Write tests for your changes
4. Ensure all tests pass (`cargo test`)
5. Run benchmarks if performance-related (`cargo bench`)
6. Submit a pull request

For major changes, please open an issue first to discuss the proposed changes.

## License

See the [LICENSE](LICENSE) file for details.

## Acknowledgments

Built with Rust 2021 edition, leveraging:
- Dijkstra's Shunting Yard algorithm for expression parsing
- RPN (Reverse Polish Notation) for efficient evaluation
- Rayon for parallel computation
- Criterion for accurate performance measurement
