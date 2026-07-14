//! Runtime catalog of every symbol the equation analyzer understands:
//! constants, unary/variadic functions, operators, log-with-base, and the
//! variable `x`. One source of truth — the tokenizer resolves names through
//! it, the evaluator dispatches through it, and downstream tools (like
//! rm-repl's `:fns`) discover the surface area through it.
//!
//! # Naming constraint
//!
//! The tokenizer matches `x`, `y`, `π`, and `e` at the **character level**,
//! before identifier scanning. An entry whose name starts with one of those
//! characters (`exp`, `xor`, …) would mis-tokenize: the leading character is
//! consumed as its single-char meaning and the rest of the name is scanned
//! separately (`exp(1)` would lex as `e`, `x`, `p(1)`). Pick a different
//! name, or teach the tokenizer to scan a full identifier before falling
//! back to single-char matches.

use crate::utilities::{abs_f32, factorial, square_root_f32};
use std::collections::HashMap;
use std::f32::consts::{E, PI};

/// A single named symbol in the equation-analyzer surface area.
///
/// Every field is `Copy + 'static` so the entire catalog can live in a `const`
/// slice. Downstream code holds `&'static Symbol` references — cheap to pass,
/// pointer-equal when they name the same entry.
#[derive(Debug)]
pub struct Symbol {
    pub name: &'static str,
    pub aliases: &'static [&'static str],
    pub category: Category,
    pub summary: &'static str,
    pub example: &'static str,
    pub kind: SymbolKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Constant,
    Arithmetic,
    Trig,
    InverseTrig,
    Hyperbolic,
    Logarithmic,
    Statistical,
    AngleConversion,
    Piping,
    Variable,
}

/// The behavior slot of a `Symbol`.
///
/// Function-pointer variants (`Unary`, `UnaryChecked`, `Variadic`) carry the
/// actual math. Purely descriptive variants (`LogBase`, `Operator`,
/// `Variable`) are documentation for tokens whose behavior lives in the
/// tokenizer/evaluator by necessity (special syntax or single-glyph parsing).
#[derive(Debug, Clone, Copy)]
pub enum SymbolKind {
    Constant(f32),
    Unary(fn(f32) -> f32),
    UnaryChecked(fn(f32) -> Result<f32, String>),
    Variadic {
        min_args: u8,
        max_args: Option<u8>,
        run: fn(&[f32]) -> Result<f32, String>,
    },
    /// `log_N(x)` — base is baked into the surface syntax; the tokenizer parses
    /// the `_N` suffix and stashes the base on the token payload.
    LogBase,
    Operator {
        glyph: &'static str,
        precedence: u8,
        assoc: Assoc,
        arity: OpArity,
    },
    /// The variable `x`.
    Variable,
}

impl SymbolKind {
    /// Takes exactly one argument, popped straight off the value stack —
    /// the shape required after `|>` and for non-framed calls.
    pub fn is_unary(&self) -> bool {
        matches!(self, SymbolKind::Unary(_) | SymbolKind::UnaryChecked(_))
    }

    /// Takes comma-separated arguments collected via a call frame
    /// (includes fixed-arity comma functions like `atan2` and `ch`).
    pub fn is_variadic(&self) -> bool {
        matches!(self, SymbolKind::Variadic { .. })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Assoc {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpArity {
    Prefix,
    Postfix,
    Binary,
    /// The `|>` pipe: infix but with an asymmetric right-hand-side (must be a
    /// bare unary function name).
    Pipe,
}

macro_rules! sym {
    (const $name:literal, [$($alias:literal),* $(,)?], $cat:ident, $summary:literal, $example:literal, $val:expr) => {
        Symbol { name: $name, aliases: &[$($alias),*], category: Category::$cat, summary: $summary, example: $example, kind: SymbolKind::Constant($val) }
    };
    (unary $name:literal, [$($alias:literal),* $(,)?], $cat:ident, $summary:literal, $example:literal, $f:expr) => {
        Symbol { name: $name, aliases: &[$($alias),*], category: Category::$cat, summary: $summary, example: $example, kind: SymbolKind::Unary($f) }
    };
    (unary_checked $name:literal, [$($alias:literal),* $(,)?], $cat:ident, $summary:literal, $example:literal, $f:expr) => {
        Symbol { name: $name, aliases: &[$($alias),*], category: Category::$cat, summary: $summary, example: $example, kind: SymbolKind::UnaryChecked($f) }
    };
    (variadic $name:literal, [$($alias:literal),* $(,)?], $cat:ident, $summary:literal, $example:literal, min: $min:literal, max: $max:expr, $f:expr) => {
        Symbol { name: $name, aliases: &[$($alias),*], category: Category::$cat, summary: $summary, example: $example, kind: SymbolKind::Variadic { min_args: $min, max_args: $max, run: $f } }
    };
    (op $name:literal, [$($alias:literal),* $(,)?], $cat:ident, $summary:literal, $example:literal, glyph: $glyph:literal, prec: $prec:literal, $assoc:ident, $arity:ident) => {
        Symbol { name: $name, aliases: &[$($alias),*], category: Category::$cat, summary: $summary, example: $example, kind: SymbolKind::Operator { glyph: $glyph, precedence: $prec, assoc: Assoc::$assoc, arity: OpArity::$arity } }
    };
    (log_base $name:literal, [$($alias:literal),* $(,)?], $cat:ident, $summary:literal, $example:literal) => {
        Symbol { name: $name, aliases: &[$($alias),*], category: Category::$cat, summary: $summary, example: $example, kind: SymbolKind::LogBase }
    };
    (variable $name:literal, [$($alias:literal),* $(,)?], $cat:ident, $summary:literal, $example:literal) => {
        Symbol { name: $name, aliases: &[$($alias),*], category: Category::$cat, summary: $summary, example: $example, kind: SymbolKind::Variable }
    };
}

pub const CATALOG: &[Symbol] = &[
    // Constants
    sym!(const "π", ["pi"], Constant, "ratio of a circle's circumference to its diameter", "π ≈ 3.14159", PI),
    sym!(const "e", [], Constant, "Euler's number", "e ≈ 2.71828", E),

    // Trigonometric
    sym!(unary "sin", [], Trig, "sine (argument in radians)", "sin(0) = 0", |x| x.sin()),
    sym!(unary "cos", [], Trig, "cosine (argument in radians)", "cos(0) = 1", |x| x.cos()),
    sym!(unary "tan", [], Trig, "tangent (argument in radians)", "tan(0) = 0", |x| x.tan()),
    sym!(unary "sec", [], Trig, "secant — 1 / cos(x)", "sec(0) = 1", |x| 1.0 / x.cos()),
    sym!(unary "csc", [], Trig, "cosecant — 1 / sin(x)", "csc(π/2) = 1", |x| 1.0 / x.sin()),
    sym!(unary "cot", [], Trig, "cotangent — 1 / tan(x)", "cot(π/4) = 1", |x| 1.0 / x.tan()),

    // Inverse trigonometric
    sym!(unary "asin", ["arcsin"], InverseTrig, "arcsine — returns radians", "asin(1) = π/2", |x| x.asin()),
    sym!(unary "acos", ["arccos"], InverseTrig, "arccosine — returns radians", "acos(1) = 0", |x| x.acos()),
    sym!(unary "atan", ["arctan"], InverseTrig, "arctangent — returns radians", "atan(0) = 0", |x| x.atan()),
    sym!(variadic "atan2", [], InverseTrig, "two-argument arctangent — atan2(y, x)", "atan2(1, 1) = π/4",
         min: 2, max: Some(2), |xs| Ok(xs[0].atan2(xs[1]))),

    // Hyperbolic
    sym!(unary "sinh", [], Hyperbolic, "hyperbolic sine", "sinh(0) = 0", |x| x.sinh()),
    sym!(unary "cosh", [], Hyperbolic, "hyperbolic cosine", "cosh(0) = 1", |x| x.cosh()),
    sym!(unary "tanh", [], Hyperbolic, "hyperbolic tangent", "tanh(0) = 0", |x| x.tanh()),

    // Angle conversion
    sym!(unary "deg", [], AngleConversion, "radians → degrees", "deg(π) = 180", |x| x * 180.0 / PI),
    sym!(unary "rad", [], AngleConversion, "degrees → radians", "rad(180) = π", |x| x * PI / 180.0),

    // Arithmetic (function form)
    sym!(unary "abs", [], Arithmetic, "absolute value", "abs(-3) = 3", abs_f32),
    sym!(unary_checked "sqrt", [], Arithmetic, "square root (NaN for negatives)", "sqrt(9) = 3",
         |x| if x.is_sign_negative() { Ok(f32::NAN) } else { Ok(square_root_f32(x)) }),

    // Logarithms
    sym!(unary "ln", [], Logarithmic, "natural log (base e)", "ln(e) = 1", |x| x.ln()),
    sym!(log_base "log", [], Logarithmic, "log with explicit base — write log_N(x)", "log_2(8) = 3"),

    // Statistical / variadic
    sym!(variadic "min", [], Statistical, "minimum of arguments", "min(3, 1, 4) = 1", min: 1, max: None,
         |xs| Ok(xs.iter().copied().fold(f32::MAX, f32::min))),
    sym!(variadic "max", [], Statistical, "maximum of arguments", "max(3, 1, 4) = 4", min: 1, max: None,
         |xs| Ok(xs.iter().copied().fold(f32::MIN, f32::max))),
    sym!(variadic "avg", [], Statistical, "arithmetic mean of arguments", "avg(2, 4, 6) = 4", min: 1, max: None,
         |xs| Ok(xs.iter().sum::<f32>() / xs.len() as f32)),
    sym!(variadic "med", [], Statistical, "median of arguments", "med(1, 3, 5) = 3", min: 1, max: None,
         |xs| {
             let mut params = xs.to_vec();
             params.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
             let len = params.len();
             let result = if len.is_multiple_of(2) {
                 let mid = len / 2;
                 (params[mid - 1] + params[mid]) / 2.0
             } else {
                 params[len / 2]
             };
             Ok(result)
         }),
    sym!(variadic "mode", [], Statistical, "most frequent value(s); NaN if all unique", "mode(1, 2, 2, 3) = 2", min: 1, max: None,
         |xs| {
             let mut seen: HashMap<u32, usize> = HashMap::new();
             for &v in xs {
                 *seen.entry(v.to_bits()).or_insert(0) += 1;
             }
             let max_count = *seen
                 .values()
                 .max()
                 .ok_or_else(|| String::from("mode requires at least one parameter"))?;
             let result = if max_count == 1 {
                 f32::NAN
             } else {
                 let modes: Vec<f32> = seen
                     .iter()
                     .filter(|(_, &c)| c == max_count)
                     .map(|(&bits, _)| f32::from_bits(bits))
                     .collect();
                 modes.iter().sum::<f32>() / modes.len() as f32
             };
             Ok(result)
         }),
    sym!(variadic "ch", [], Statistical, "binomial coefficient — ch(n, k) = n choose k", "ch(5, 2) = 10", min: 2, max: Some(2),
         |xs| {
             for (i, &v) in xs.iter().enumerate() {
                 if v % 1.0 != 0.0 {
                     return Err(format!("Parameter {} must be an integer, got {}", i + 1, v));
                 }
                 if v < 0.0 {
                     return Err(format!("Parameter {} must be non-negative, got {}", i + 1, v));
                 }
             }
             let n = xs[0] as isize;
             let k = xs[1] as isize;
             let result = if k > n {
                 0.0
             } else {
                 (factorial(n) / (factorial(k) * factorial(n - k))) as f32
             };
             Ok(result)
         }),

    // Operators (docs + precedence/assoc — dispatch stays glyph-tokenized in evaluator)
    sym!(op "+", [], Arithmetic, "addition", "2 + 3 = 5", glyph: "+", prec: 2, Left, Binary),
    sym!(op "-", [], Arithmetic, "subtraction (or unary negation)", "5 - 2 = 3", glyph: "-", prec: 2, Left, Binary),
    sym!(op "*", [], Arithmetic, "multiplication", "2 * 3 = 6", glyph: "*", prec: 3, Left, Binary),
    sym!(op "/", [], Arithmetic, "division", "6 / 2 = 3", glyph: "/", prec: 3, Left, Binary),
    sym!(op "^", [], Arithmetic, "exponentiation", "2 ^ 3 = 8", glyph: "^", prec: 4, Right, Binary),
    sym!(op "%", [], Arithmetic, "percent (postfix): x% = x/100; after + or -, b% is a percentage of the left side", "100 - 20% = 80", glyph: "%", prec: 5, Left, Postfix),
    sym!(op "mod", ["%%"], Arithmetic, "modulo (remainder)", "17 mod 5 = 2", glyph: "mod", prec: 3, Left, Binary),
    sym!(op "!", [], Arithmetic, "factorial (postfix)", "5! = 120", glyph: "!", prec: 5, Left, Postfix),
    sym!(op "|>", ["|"], Piping, "pipe: pass LHS as sole argument to a unary function on the RHS", "π/2 |> sin", glyph: "|>", prec: 1, Left, Pipe),

    // Variable
    sym!(variable "x", [], Variable, "the running variable — set by plot(), 0 in calculate()", "y = x^2"),
];

/// Every symbol the equation analyzer understands.
pub fn all() -> &'static [Symbol] {
    CATALOG
}

/// Look up a symbol by its canonical name or any alias. Linear scan; the
/// catalog is small and this is called at most once per identifier while
/// tokenizing.
pub fn find(name: &str) -> Option<&'static Symbol> {
    CATALOG
        .iter()
        .find(|s| s.name == name || s.aliases.contains(&name))
}

/// Every symbol in a given category, in catalog declaration order.
pub fn by_category(cat: Category) -> impl Iterator<Item = &'static Symbol> {
    CATALOG.iter().filter(move |s| s.category == cat)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn catalog_is_non_empty() {
        assert!(!CATALOG.is_empty());
    }

    #[test]
    fn every_entry_has_non_empty_summary_and_example() {
        for s in CATALOG {
            assert!(!s.summary.is_empty(), "empty summary on '{}'", s.name);
            assert!(!s.example.is_empty(), "empty example on '{}'", s.name);
        }
    }

    #[test]
    fn names_and_aliases_are_globally_unique() {
        let mut seen: Vec<&'static str> = Vec::new();
        for s in CATALOG {
            for label in std::iter::once(s.name).chain(s.aliases.iter().copied()) {
                assert!(
                    !seen.contains(&label),
                    "duplicate label '{label}' — every name/alias must be unique across CATALOG"
                );
                seen.push(label);
            }
        }
    }

    #[test]
    fn alias_lookup_matches_canonical() {
        // Pointer identity — an alias must resolve to the same &'static Symbol
        // as its canonical name.
        let asin = find("asin").unwrap();
        let arcsin = find("arcsin").unwrap();
        assert!(std::ptr::eq(asin, arcsin));

        let atan = find("atan").unwrap();
        let arctan = find("arctan").unwrap();
        assert!(std::ptr::eq(atan, arctan));

        let pi = find("π").unwrap();
        let pi_alias = find("pi").unwrap();
        assert!(std::ptr::eq(pi, pi_alias));
    }

    #[test]
    fn find_returns_none_for_unknown() {
        assert!(find("thereisnowaythisexists").is_none());
    }

    #[test]
    fn by_category_returns_expected_membership() {
        assert!(by_category(Category::Trig).count() >= 6); // sin cos tan sec csc cot
        assert!(by_category(Category::InverseTrig).count() >= 4); // asin acos atan atan2
        assert!(by_category(Category::Constant).count() >= 2); // π e
        assert!(by_category(Category::Statistical).count() >= 6); // min max avg med mode ch
        assert!(by_category(Category::Variable).count() >= 1); // x
    }

    #[test]
    fn variadic_min_args_at_least_one() {
        for s in CATALOG {
            if let SymbolKind::Variadic { min_args, .. } = s.kind {
                assert!(
                    min_args >= 1,
                    "'{}' has min_args = 0 — a zero-arg call is not variadic",
                    s.name
                );
            }
        }
    }

    #[test]
    fn variadic_max_ge_min_when_set() {
        for s in CATALOG {
            if let SymbolKind::Variadic {
                min_args,
                max_args: Some(max),
                ..
            } = s.kind
            {
                assert!(
                    max >= min_args,
                    "'{}' has max_args ({}) < min_args ({})",
                    s.name,
                    max,
                    min_args
                );
            }
        }
    }

    #[test]
    fn known_math_dispatches_correctly() {
        // Smoke-test a few closures directly (not via the pipeline yet).
        let sin = find("sin").unwrap();
        if let SymbolKind::Unary(f) = sin.kind {
            assert!((f(0.0) - 0.0).abs() < 1e-6);
        } else {
            panic!("sin should be Unary");
        }

        let sqrt = find("sqrt").unwrap();
        if let SymbolKind::UnaryChecked(f) = sqrt.kind {
            assert_eq!(f(9.0).unwrap(), 3.0);
            assert!(f(-1.0).unwrap().is_nan());
        } else {
            panic!("sqrt should be UnaryChecked");
        }

        let avg = find("avg").unwrap();
        if let SymbolKind::Variadic { run, .. } = avg.kind {
            assert_eq!(run(&[2.0, 4.0, 6.0]).unwrap(), 4.0);
        } else {
            panic!("avg should be Variadic");
        }

        let ch = find("ch").unwrap();
        if let SymbolKind::Variadic { run, .. } = ch.kind {
            assert_eq!(run(&[5.0, 2.0]).unwrap(), 10.0);
            assert!(run(&[5.0, 2.5]).is_err()); // non-integer
        } else {
            panic!("ch should be Variadic");
        }
    }

    #[test]
    fn operator_precedence_matches_operands_module() {
        // Regression guard: catalog operator precedence must stay in sync with
        // structs::operands (until we drive operands from the catalog in task 10).
        let expect = |name: &str, prec: u8| {
            let sym = find(name).unwrap_or_else(|| panic!("missing operator {name}"));
            match sym.kind {
                SymbolKind::Operator { precedence, .. } => assert_eq!(
                    precedence, prec,
                    "operator '{name}' precedence mismatch"
                ),
                _ => panic!("'{name}' is not an Operator"),
            }
        };
        expect("+", 2);
        expect("-", 2);
        expect("*", 3);
        expect("/", 3);
        expect("mod", 3);
        expect("%%", 3); // alias of mod
        expect("^", 4);
        expect("!", 5);
        expect("%", 5); // postfix, binds like factorial
    }
}


