# rusty-maths

[![Rust](https://github.com/ShaneMarusczak/rusty-maths/actions/workflows/rust.yml/badge.svg?branch=main&event=push)](https://github.com/ShaneMarusczak/rusty-maths/actions/workflows/rust.yml)

A Rust math library. The centerpiece is the **equation analyzer** — an
expression engine with spanned errors and user-definable functions — backed
by smaller statistics, linear algebra, geometry, gradient descent, and
neural network modules.

Used as the engine behind [rm-repl](https://github.com/ShaneMarusczak/rm-repl).

## Equation analyzer

Evaluate expressions or plot them over a range:

```rust
use rusty_maths::equation_analyzer::calculator;

let result = calculator::calculate("2 + 3 * 4").unwrap();
assert_eq!(result, 14.0);

let points = calculator::plot("x^2 - 2x + 1", -5.0, 5.0, 0.1).unwrap();
```

### User definitions

`calculate_with` / `plot_with` evaluate against a set of named values and
single-parameter functions. Function bodies are stored as source and read
other definitions at *call* time:

```rust
use rusty_maths::equation_analyzer::{calculator::calculate_with, Definitions};

let mut defs = Definitions::new();
defs.define_value("a", 3.0).unwrap();
defs.define_function("g", "a * x^2").unwrap();

assert_eq!(calculate_with("g(2) + 1", &defs).unwrap(), 13.0);
assert_eq!(calculate_with("4 |> g", &defs).unwrap(), 48.0);

defs.define_value("a", 1.0).unwrap(); // g sees the new value — late binding
assert_eq!(calculate_with("g(2)", &defs).unwrap(), 4.0);
```

Recursion is depth-capped, names can't shadow built-ins, and a broken
definition only errors if actually called.

### Errors

Every error is an `EquationError` carrying a message, an optional
**character span** into the source (what rm-repl renders carets with), and
— for errors inside a user function's body — the function's name via
`in_function`. Bad function calls suggest the nearest name:

```rust
let err = calculator::calculate("2 + sinq(3)").unwrap_err();
assert_eq!(err.message, "Invalid function name sinq — did you mean 'sin'?");
assert_eq!(err.span.map(|s| (s.start, s.end)), Some((4, 8)));
```

### The catalog

Every symbol the analyzer understands lives in one registry,
`equation_analyzer::catalog` — iterate `catalog::all()` or look up
`catalog::find("sin")`. At a glance:

- Operators: `+`, `-`, `*`, `/`, `^`, `!`, `mod` (also `%%`), postfix `%`
  (`50%` = 0.5; after `+`/`-` it's a percentage of the left operand, so
  `100 - 20%` = 80), and `|>` pipe (`π/2 |> sin`)
- Trig / inverse / hyperbolic: `sin`, `cos`, `tan`, `sec`, `csc`, `cot`,
  `asin`, `acos`, `atan` (+ `arc*` aliases), `atan2`, `sinh`, `cosh`,
  `tanh`, `asinh`, `acosh`, `atanh`
- Arithmetic: `abs`, `sqrt`, `root(x, n)` (real odd roots of negatives),
  `pow`, `floor`, `ceil`, `round`
- Logarithms: `ln`, `exp`, `log_N(x)`
- Statistical: `min`, `max`, `sum`, `avg`, `med`, `mode`, `ch`, `perm`
  (the counting pair computes multiplicatively — `ch(1000, 3)` works)
- Angle conversion: `deg`, `rad`; constants `π` (`pi`), `e`
- Variable `x` with coefficient support (`2x`, `-3x^2`)

### Pipeline

Streaming tokenizer → Shunting Yard parser → stack-based RPN evaluator;
plotting evaluates points in parallel with Rayon. Details in the
[pipeline README](src/equation_analyzer/pipeline/README.md); benchmarks via
`cargo bench --bench equation_analyzer`.

## Other modules

**Statistics** — `mean`, `median`, `variance`, `standard_deviation`,
`correlation`, `quantile`, `interquartile_range`, and friends.

**Linear algebra** — vector/matrix helpers: `dot_product`, `vec_add`,
`scalar_multiply`, `magnitude`, `distance`, `vector_sum`, `vector_mean`.

**Geometry** — `Circle` with area/circumference and related calculations.

**Gradient descent** — `linear_gradient` and `mini_batches` optimizers.

**Neural network** — small `Network` of `Dense` + activation layers (ReLU,
Sigmoid, Tanh, Linear) with backprop, MSE loss, and Xavier init. Learning
demos (XOR, regression, sine approximation) live behind `#[ignore]`:
`cargo test -- --ignored --nocapture`.

## Installation

```toml
[dependencies]
rusty-maths = { git = "https://github.com/ShaneMarusczak/rusty-maths.git" }
```

## Testing

```bash
cargo test
```

500+ tests cover the analyzer (tokenizing, parsing, evaluation, plotting,
definitions, error spans) and the supporting modules.

## License

See the [LICENSE](LICENSE) file for details.
