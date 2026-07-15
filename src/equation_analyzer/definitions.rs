//! User definitions — named values and single-parameter functions — that
//! extend the equation analyzer's vocabulary at evaluation time.
//!
//! Build a [`Definitions`] set, then evaluate against it with
//! [`calculate_with`](crate::equation_analyzer::calculator::calculate_with)
//! or [`plot_with`](crate::equation_analyzer::calculator::plot_with):
//!
//! ```
//! use rusty_maths::equation_analyzer::calculator::calculate_with;
//! use rusty_maths::equation_analyzer::Definitions;
//!
//! let mut defs = Definitions::new();
//! defs.define_value("a", 3.0).unwrap();
//! defs.define_function("g", "a * x^2").unwrap();
//!
//! assert_eq!(calculate_with("g(2) + 1", &defs).unwrap(), 13.0);
//! ```
//!
//! Names live in a single namespace: defining a value and then a function
//! under the same name replaces the value, and vice versa. Catalog names
//! (`sin`, `pi`, …) and the reserved letters `x`/`y` cannot be redefined.
//!
//! Function bodies are stored as **source text** and resolved late: a body
//! referencing `a` sees whatever `a` is bound to when the function is
//! *called*, not when it was defined. The single parameter is always `x` —
//! bodies parse exactly like top-level equations.

use crate::equation_analyzer::catalog;
use crate::equation_analyzer::errors::EquationError;
use crate::equation_analyzer::pipeline::parser::parse;
use crate::equation_analyzer::pipeline::tokenizer::StreamingTokenizer;
use crate::equation_analyzer::structs::token::SpannedToken;

/// A set of user definitions, in definition order.
#[derive(Debug, Clone, Default)]
pub struct Definitions {
    entries: Vec<Entry>,
}

#[derive(Debug, Clone)]
struct Entry {
    name: String,
    kind: DefKind,
}

#[derive(Debug, Clone)]
enum DefKind {
    Value(f32),
    Function { body: String },
}

/// A read-only view of one definition, for listing (`:fns`-style output)
/// and persistence.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Definition<'a> {
    Value { name: &'a str, value: f32 },
    Function { name: &'a str, body: &'a str },
}

/// What an identifier resolved to, for the tokenizer.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Resolved {
    Value(f32),
    Function(usize),
}

impl Definitions {
    pub fn new() -> Self {
        Self::default()
    }

    /// Defines (or redefines) a named value.
    ///
    /// Fails if the name is not a valid identifier or collides with a
    /// catalog entry or reserved letter; on failure the set is unchanged.
    pub fn define_value(&mut self, name: &str, value: f32) -> Result<(), EquationError> {
        validate_name(name)?;
        self.upsert(name, DefKind::Value(value));
        Ok(())
    }

    /// Defines (or redefines) a named single-parameter function. `body` is
    /// stored as source text; its parameter is always `x`.
    ///
    /// Only the *name* is checked here (a body may legally reference names
    /// defined later) — call [`validate_function`](Self::validate_function)
    /// afterwards to check that the body currently compiles.
    pub fn define_function(&mut self, name: &str, body: &str) -> Result<(), EquationError> {
        validate_name(name)?;
        if body.trim().is_empty() {
            return Err(EquationError::new(format!(
                "Function '{name}' has an empty body"
            )));
        }
        self.upsert(
            name,
            DefKind::Function {
                body: body.trim().to_string(),
            },
        );
        Ok(())
    }

    /// Checks that the named function's body tokenizes and parses against
    /// the current definitions. The error is tagged with the function name
    /// and its span refers to the body source.
    ///
    /// This is a compile check, not an evaluation: operand-count problems
    /// that shunting-yard accepts structurally (`x +`, `x x`) only surface
    /// when the function is called.
    pub fn validate_function(&self, name: &str) -> Result<(), EquationError> {
        let body = self
            .function_body(name)
            .ok_or_else(|| EquationError::new(format!("No function named '{name}' is defined")))?;
        StreamingTokenizer::new_with(body, Some(self))
            .and_then(parse)
            .map(|_| ())
            .map_err(|e| e.for_function(name))
    }

    /// Removes a definition by name. Returns whether one existed.
    pub fn undefine(&mut self, name: &str) -> bool {
        match self.index_of(name) {
            Some(i) => {
                self.entries.remove(i);
                true
            }
            None => false,
        }
    }

    /// The value bound to `name`, if it is a value definition.
    pub fn value(&self, name: &str) -> Option<f32> {
        match self.find(name)? {
            DefKind::Value(v) => Some(*v),
            DefKind::Function { .. } => None,
        }
    }

    /// The body source of `name`, if it is a function definition.
    pub fn function_body(&self, name: &str) -> Option<&str> {
        match self.find(name)? {
            DefKind::Function { body } => Some(body),
            DefKind::Value(_) => None,
        }
    }

    /// Whether any definition (value or function) exists under `name`.
    pub fn contains(&self, name: &str) -> bool {
        self.index_of(name).is_some()
    }

    /// All definitions, in definition order.
    pub fn iter(&self) -> impl Iterator<Item = Definition<'_>> {
        self.entries.iter().map(|e| match &e.kind {
            DefKind::Value(v) => Definition::Value {
                name: &e.name,
                value: *v,
            },
            DefKind::Function { body } => Definition::Function {
                name: &e.name,
                body,
            },
        })
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Resolves an identifier for the tokenizer. Catalog resolution happens
    /// first at the call site; this only sees names the catalog didn't claim.
    pub(crate) fn resolve(&self, name: &str) -> Option<Resolved> {
        let i = self.index_of(name)?;
        match &self.entries[i].kind {
            DefKind::Value(v) => Some(Resolved::Value(*v)),
            DefKind::Function { .. } => Some(Resolved::Function(i)),
        }
    }

    /// Compiles every function body once, for one evaluation pass. Broken
    /// bodies are stored as errors and surface only if actually called —
    /// which is what makes late binding observable: an equation that never
    /// calls a broken function is unaffected by it.
    pub(crate) fn compile(&self) -> CompiledDefinitions<'_> {
        let bodies = self
            .entries
            .iter()
            .map(|e| match &e.kind {
                DefKind::Value(_) => None,
                DefKind::Function { body } => {
                    Some(StreamingTokenizer::new_with(body, Some(self)).and_then(parse))
                }
            })
            .collect();
        CompiledDefinitions { defs: self, bodies }
    }

    fn find(&self, name: &str) -> Option<&DefKind> {
        self.index_of(name).map(|i| &self.entries[i].kind)
    }

    // Linear scan: definition sets are user-typed and tiny, and this keeps
    // definition order (needed by iter/persistence) in one flat Vec.
    fn index_of(&self, name: &str) -> Option<usize> {
        self.entries.iter().position(|e| e.name == name)
    }

    fn upsert(&mut self, name: &str, kind: DefKind) {
        match self.index_of(name) {
            Some(i) => self.entries[i].kind = kind,
            None => self.entries.push(Entry {
                name: name.to_string(),
                kind,
            }),
        }
    }
}

/// One evaluation pass's compiled view of a `Definitions` set: each function
/// body tokenized and parsed to RPN exactly once, indexed in step with the
/// entries (values hold `None`).
pub(crate) struct CompiledDefinitions<'d> {
    defs: &'d Definitions,
    bodies: Vec<Option<Result<Vec<SpannedToken>, EquationError>>>,
}

impl CompiledDefinitions<'_> {
    /// The definition's name, for error messages.
    pub(crate) fn name(&self, index: usize) -> &str {
        self.defs
            .entries
            .get(index)
            .map_or("?", |e| e.name.as_str())
    }

    /// The compiled body RPN for a function definition.
    pub(crate) fn body_rpn(&self, index: usize) -> Result<&[SpannedToken], EquationError> {
        match self.bodies.get(index) {
            Some(Some(Ok(rpn))) => Ok(rpn),
            Some(Some(Err(e))) => Err(e.clone()),
            _ => Err(EquationError::new(
                "Internal error: call to an unknown user definition",
            )),
        }
    }
}

/// Names follow the tokenizer's identifier rules (alphabetic start,
/// alphanumeric continuation) and must not shadow anything built in.
fn validate_name(name: &str) -> Result<(), EquationError> {
    let mut chars = name.chars();
    let valid_shape = chars.next().is_some_and(char::is_alphabetic)
        && chars.all(|c| c.is_alphabetic() || c.is_ascii_digit());
    if !valid_shape {
        return Err(EquationError::new(format!(
            "Invalid name '{name}': names start with a letter and contain only letters and digits"
        )));
    }
    if name == "x" || name == "y" {
        return Err(EquationError::new(format!(
            "'{name}' is reserved (the plot variable and equation marker)"
        )));
    }
    if catalog::find(name).is_some() {
        return Err(EquationError::new(format!(
            "Cannot redefine built-in '{name}'"
        )));
    }
    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn define_and_read_back() {
        let mut defs = Definitions::new();
        defs.define_value("a", 3.0).unwrap();
        defs.define_function("g", " 2x^2 ").unwrap();

        assert_eq!(defs.value("a"), Some(3.0));
        assert_eq!(defs.function_body("g"), Some("2x^2"));
        assert_eq!(defs.value("g"), None);
        assert_eq!(defs.function_body("a"), None);
        assert!(defs.contains("a") && defs.contains("g"));
        assert_eq!(defs.len(), 2);
    }

    #[test]
    fn redefine_replaces_across_kinds() {
        let mut defs = Definitions::new();
        defs.define_value("a", 3.0).unwrap();
        defs.define_function("a", "x + 1").unwrap();
        assert_eq!(defs.value("a"), None);
        assert_eq!(defs.function_body("a"), Some("x + 1"));
        assert_eq!(defs.len(), 1);
    }

    #[test]
    fn undefine_removes() {
        let mut defs = Definitions::new();
        defs.define_value("a", 3.0).unwrap();
        assert!(defs.undefine("a"));
        assert!(!defs.undefine("a"));
        assert!(defs.is_empty());
    }

    #[test]
    fn names_are_validated() {
        let mut defs = Definitions::new();
        assert!(defs.define_value("2a", 1.0).is_err());
        assert!(defs.define_value("a b", 1.0).is_err());
        assert!(defs.define_value("", 1.0).is_err());
        assert!(defs.define_value("x", 1.0).is_err());
        assert!(defs.define_value("y", 1.0).is_err());
        assert!(defs.define_value("sin", 1.0).is_err());
        assert!(defs.define_value("pi", 1.0).is_err());
        assert!(defs.define_function("mod", "x").is_err());
        assert!(defs.is_empty());
    }

    #[test]
    fn empty_body_rejected() {
        let mut defs = Definitions::new();
        assert!(defs.define_function("g", "   ").is_err());
    }

    #[test]
    fn validate_function_checks_current_compile() {
        let mut defs = Definitions::new();
        defs.define_function("g", "sin(2x").unwrap();
        let err = defs.validate_function("g").unwrap_err();
        assert_eq!(err.in_function.as_deref(), Some("g"));

        defs.define_function("h", "a * x").unwrap();
        // 'a' is not defined yet — late binding means this is an error now...
        assert!(defs.validate_function("h").is_err());
        // ...but defining 'a' cures it.
        defs.define_value("a", 1.0).unwrap();
        assert!(defs.validate_function("h").is_ok());
    }

    #[test]
    fn iter_preserves_definition_order() {
        let mut defs = Definitions::new();
        defs.define_value("a", 1.0).unwrap();
        defs.define_function("g", "x").unwrap();
        let listed: Vec<Definition> = defs.iter().collect();
        assert_eq!(
            listed,
            vec![
                Definition::Value {
                    name: "a",
                    value: 1.0
                },
                Definition::Function {
                    name: "g",
                    body: "x"
                },
            ]
        );
    }
}
