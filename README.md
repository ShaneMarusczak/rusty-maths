# rusty-maths

[![Rust](https://github.com/ShaneMarusczak/rusty-maths/actions/workflows/rust.yml/badge.svg?branch=main&event=push)](https://github.com/ShaneMarusczak/rusty-maths/actions/workflows/rust.yml)

A high-performance Rust library for mathematical operations, featuring an advanced equation analyzer with multiple pipeline architectures optimized for different use cases.

## Features

### 🚀 High-Performance Equation Analyzer

Three pipeline implementations offering different performance/flexibility trade-offs:

- **Vec Pipeline**: Traditional fully-buffered implementation (baseline)
- **Hybrid Pipeline**: Streaming tokenizer with buffered parser - **1.6x faster**
- **Full Pipeline**: Fully streaming architecture with minimal buffers - **1.4x faster**

All pipelines share the same core algorithms through a DRY architecture, ensuring consistent behavior while optimizing for different use cases.

```rust
use rusty_maths::equation_analyzer::hybrid_pipeline::calculator;

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

The equation analyzer has been extensively optimized:

**Benchmark Results** (equation: "2 + 3")
- Vec Pipeline: 329ns (baseline)
- Hybrid Pipeline: 194ns (**41% faster**)
- Full Pipeline: 253ns (**23% faster**)

**Plot Performance** (1000 points, equation: "x^2 + 2x + 1")
- Parallel evaluation using Rayon
- Parse once, evaluate many times
- Scales efficiently with point count

Run benchmarks yourself:
```bash
cargo bench --bench equation_analyzer
```

## Architecture

### Equation Analyzer Pipeline Comparison

```
┌──────────────────────────────────────────────────────┐
│                   core/                              │
│  ┌─────────────┐  ┌──────────┐  ┌──────────────┐   │
│  │ Tokenizers  │  │  Parsers  │  │  Evaluator   │   │
│  │ - Vec       │  │ - Shunting│  │  - Generic   │   │
│  │ - Streaming │  │ - Streaming│  │    RPN      │   │
│  └─────────────┘  └──────────┘  └──────────────┘   │
└──────────────────────────────────────────────────────┘
            ▲              ▲              ▲
            └──────────────┴──────────────┘
                   Shared by all pipelines
         ┌──────────────┬──────────────┬──────────────┐
    vec_pipeline   hybrid_pipeline   full_pipeline
```

**Vec Pipeline** - Traditional fully-buffered approach
- Best for: Debugging, learning, baseline reference
- Characteristics: Complete buffering at each stage

**Hybrid Pipeline** - Streaming input, buffered output
- Best for: Production use, drop-in performance improvement
- Characteristics: Iterator-based tokenizer, Vec-based parser
- Performance: 1.6x faster than baseline

**Full Pipeline** - Fully streaming with minimal buffers
- Best for: Architectural elegance, early termination scenarios
- Characteristics: All stages are iterators, minimal partial buffers
- Performance: 1.4x faster than baseline

See individual pipeline READMEs for detailed architecture documentation:
- [Vec Pipeline](src/equation_analyzer/vec_pipeline/README.md)
- [Hybrid Pipeline](src/equation_analyzer/hybrid_pipeline/README.md)
- [Full Pipeline](src/equation_analyzer/full_pipeline/README.md)

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
use rusty_maths::equation_analyzer::hybrid_pipeline::calculator;
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

**180 tests** covering:
- 136 equation analyzer tests (cross-validated across all pipelines)
- 44 tests for statistics, linear algebra, geometry, and gradient descent
- All tests ensure behavior consistency across pipeline implementations

## Code Quality

The codebase emphasizes:
- **DRY Principle**: Shared core algorithms eliminate duplication (62% code reduction)
- **Zero-Cost Abstractions**: Generic implementations with zero runtime overhead
- **Comprehensive Testing**: Cross-validation ensures all pipelines behave identically
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
│   │   ├── core/               # Shared implementations (DRY)
│   │   ├── vec_pipeline/       # Fully-buffered pipeline
│   │   ├── hybrid_pipeline/    # Hybrid streaming pipeline
│   │   └── full_pipeline/      # Fully streaming pipeline
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
