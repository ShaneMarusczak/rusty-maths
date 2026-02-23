# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Build
cargo build
cargo build --release

# Test (runs ~436 tests)
cargo test

# Run a single test by name
cargo test <test_name>

# Run ignored (long-running neural network) tests
cargo test xor_learning_test -- --ignored --nocapture
cargo test -- --ignored           # all ignored tests
cargo test -- --include-ignored   # all tests including ignored

# Benchmarks
cargo bench --bench equation_analyzer

# Lint (Clippy is configured with extra warnings)
cargo clippy
```

## Architecture

The library exposes six top-level modules from `src/lib.rs`: `equation_analyzer`, `geometry`, `gradient_descent`, `linear_algebra`, `neural_network`, `statistics`, and `utilities`.

### Equation Analyzer Pipeline

The core of the library. The public API is `equation_analyzer::calculator::{calculate, plot}`.

Internally it is a three-stage pipeline:

```
StreamingTokenizer (Iterator<Item=Result<Token,String>>)
    → parse() → Vec<Token> in RPN
    → evaluate(tokens, x) → f32
```

- **Tokenizer** (`pipeline/tokenizer.rs`): `StreamingTokenizer` is a lazy `Iterator` over a `&str`. It handles unary vs binary minus disambiguation, implicit coefficient syntax (`2x` → `Number(2) * X(1,1)`), and `log_N(...)` syntax. It emits an `End` token at the end of the stream.
- **Parser** (`pipeline/parser.rs`): `parse<I>()` is generic over any `IntoIterator<Item=Result<Token,String>>`. Implements Dijkstra's Shunting Yard algorithm to convert infix tokens to RPN (`Vec<Token>`). Handles variadic functions (`avg`, `min`, `max`, `med`, `mode`, `ch`) via a frame stack and synthetic `End*` tokens.
- **Evaluator** (`pipeline/evaluator.rs`): `evaluate<I>()` consumes RPN tokens with an optional `x` value. Uses a `Vec<f32>` stack and a `FunctionFrame` stack for variadic functions.
- **Token** (`structs/token.rs`): `Token` holds a `TokenType` + two `f32` fields. For `X` tokens, `numeric_value_1` = coefficient and `numeric_value_2` = exponent. For `Log`, `numeric_value_1` = base. Synthetic `End*` tokens (`EndAvg`, `EndMin`, etc.) are produced by the parser, never by the tokenizer.
- `plot()` parses once and then evaluates in parallel via Rayon over the x-values.

### Neural Network

`neural_network/`: `Network` owns a `Vec<Box<dyn Layer>>`. `Dense` layers and `ActivationLayer` wrappers implement the `Layer` trait (forward/backward pass + parameter update). Xavier weight initialization. Long-running learning tests are `#[ignore]`d.

### Other Modules

- `statistics/`: standalone functions (`mean`, `median`, `standard_deviation`, `correlation`, etc.)
- `linear_algebra/`: vector operations (`dot_product`, `vec_add`, `scalar_multiply`, etc.)
- `geometry/`: `Circle` struct implementing a `Shape` trait
- `gradient_descent/`: `linear_gradient` and `mini_batch` functions
- `utilities/`: shared helpers (`abs_f32`, `square_root_f32`, `factorial`)

## Clippy Lints

`Cargo.toml` enables `clippy::panic`, `clippy::unwrap_used`, and `clippy::expect_used` as warnings. Prefer `?` propagation or explicit error handling over panicking calls.
