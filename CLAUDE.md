# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Build
cargo build
cargo build --release

# Test (runs ~480 tests)
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

- **Tokenizer** (`pipeline/tokenizer.rs`): `StreamingTokenizer` is a lazy `Iterator` over a `&str`; its char iterator is the only cursor (positions are counted in chars, for error messages only). It scans full identifiers before any single-character meaning applies, and handles unary vs binary minus disambiguation, juxtaposed coefficients (`2x` expands to `Number(2) * X`, parenthesized when the preceding operator binds at least as tightly as `*` — the binding is context-sensitive by design, see `coefficient_x`), and `log_N(...)` syntax. It emits an `End` token at the end of the stream.
- **Parser** (`pipeline/parser.rs`): `parse<I>()` is generic over any `IntoIterator<Item=Result<SpannedToken,EquationError>>`. Implements Dijkstra's Shunting Yard algorithm to convert infix tokens to RPN. Every parenthesized call — unary and variadic alike — becomes a `CallStart`…`EndCall` frame whose arity the evaluator enforces from the catalog; a comma is valid only as an argument separator directly inside a call (digit grouping is `1_000`). Operator precedence/associativity comes from the catalog via `structs/operands.rs` (`UnaryMinus`, which the catalog doesn't name, mirrors `^`).
- **Evaluator** (`pipeline/evaluator.rs`): `evaluate<I>()` consumes RPN tokens with an optional `x` value. Uses a value stack and a `FunctionFrame` stack; `EndCall` arity-checks (unary = exactly 1, dispatched without an argument buffer) and dispatches through the `&'static Symbol`. A bare `Call` token is a pipe target applying to the stack top.
- **Errors** (`errors.rs`): `EquationError { message, span: Option<Span> }` — spans are char-indexed half-open ranges into the source, so consumers (rm-repl) can render carets. `Display` appends a 1-based position.
- **Token** (`structs/token.rs`): an enum whose variants carry their own payloads — `Number(f32)`, `Log { base }`, `X`, and `Call`/`CallStart`/`EndCall`/`Constant` holding a `&'static Symbol` (compared by catalog identity). Tokens travel as `SpannedToken` (token + source span). `CallStart`/`EndCall` are produced by the parser, never by the tokenizer.
- `plot()` parses once and then evaluates in parallel via Rayon over the x-values. It rejects non-positive/NaN step sizes; factorial (`!`, `ch`) is limited to n ≤ 20 and errors past that.

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
