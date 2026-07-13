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

Every symbol the analyzer understands lives in one authoritative registry,
`equation_analyzer::catalog`. Iterate `catalog::all()` for the complete
list with categories, one-line summaries, and examples, or look up an
individual entry with `catalog::find("sin")`. The rm-repl companion
exposes this at runtime via `:fns` (list all) or `:fns <name>` (show one).

At a glance (see the catalog for the authoritative list):
- Arithmetic operators: `+`, `-`, `*`, `/`, `^`, `%` (percent-of),
  `%%` (modulo), `!` (factorial), `|>` (pipe)
- Trig / inverse trig / hyperbolic: sin, cos, tan, sec, csc, cot, asin
  (arcsin), acos (arccos), atan (arctan), atan2, sinh, cosh, tanh
- Angle conversion: deg (rad → °), rad (° → rad)
- Logarithms: ln, log_N
- Statistical: min, max, avg, med, mode, ch
- Constants: π (pi), e
- Variable: x, with coefficient support (`2x`, `-3x^2`)

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

### 🧠 Neural Networks

Simple but powerful neural network implementation with multiple layer types:
```rust
use rusty_maths::neural_network::{Network, Dense, ActivationLayer, ReLU, Sigmoid};

// Create a neural network
let mut network = Network::new();
network.add(Box::new(Dense::new(2, 4)));              // Input: 2, Hidden: 4
network.add(Box::new(ActivationLayer::new(ReLU, 4)));
network.add(Box::new(Dense::new(4, 1)));              // Output: 1
network.add(Box::new(ActivationLayer::new(Sigmoid, 1)));

// Training data (XOR problem)
let inputs = vec![
    vec![0.0, 0.0], vec![0.0, 1.0],
    vec![1.0, 0.0], vec![1.0, 1.0],
];
let targets = vec![vec![0.0], vec![1.0], vec![1.0], vec![0.0]];

// Train the network
let losses = network.train(&inputs, &targets, 0.1, 1000);

// Make predictions
let prediction = network.predict(&vec![1.0, 0.0]);
```

**Supported Features:**
- Dense (Fully Connected) layers
- Activation functions: ReLU, Sigmoid, Tanh, Linear
- Backpropagation with gradient descent
- Mean Squared Error loss
- Xavier/Glorot weight initialization

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
use rusty_maths::neural_network::{Network, Dense, ActivationLayer, ReLU};

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

    // Neural network
    let mut network = Network::new();
    network.add(Box::new(Dense::new(2, 3)));
    network.add(Box::new(ActivationLayer::new(ReLU, 3)));
    network.add(Box::new(Dense::new(3, 1)));

    let prediction = network.predict(&vec![0.5, 0.5]);
    println!("Prediction: {:?}", prediction);

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

**436 tests** covering:
- Equation analyzer tests (parsing, evaluation, plotting)
- Neural network tests (layers, activations, training)
- Statistics, linear algebra, geometry, and gradient descent
- Edge cases and error handling

### Long-Running Tests

Some tests are marked with `#[ignore]` to prevent them from slowing down regular test runs. These demonstrate the neural network's learning capabilities:

**Available Learning Tests:**
- `xor_learning_test` - XOR gate (non-linearly separable, requires hidden layer)
- `and_gate_test` - AND gate (linearly separable)
- `or_gate_test` - OR gate (linearly separable)
- `linear_regression_test` - Learn y = 2x + 1
- `sine_approximation_test` - Approximate sin(x) function
- `circle_classification_test` - Classify points inside/outside unit circle

```bash
# Run a specific test
cargo test xor_learning_test -- --ignored --nocapture

# Run all neural network learning tests
cargo test neural_network -- --ignored --nocapture

# Run all ignored tests
cargo test -- --ignored

# Run all tests including ignored ones
cargo test -- --include-ignored
```

Each test trains a network and verifies it learns the target function with detailed output showing training progress and predictions.

## Code Quality

The codebase emphasizes:
- **Clean Architecture**: Single, optimized pipeline implementation
- **Zero-Cost Abstractions**: Generic implementations with zero runtime overhead
- **Comprehensive Testing**: 436 tests covering all functionality
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
│   ├── neural_network/         # Neural network implementation
│   │   ├── activations.rs      # Activation functions (ReLU, Sigmoid, etc.)
│   │   ├── layer.rs            # Layer trait and implementations
│   │   └── network.rs          # Network training and inference
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
