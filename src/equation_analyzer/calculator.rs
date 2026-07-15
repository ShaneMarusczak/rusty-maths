use crate::equation_analyzer::definitions::Definitions;
use crate::equation_analyzer::errors::EquationError;
use crate::equation_analyzer::pipeline::evaluator::evaluate_with;
use crate::equation_analyzer::pipeline::parser::parse;
use crate::equation_analyzer::pipeline::tokenizer::StreamingTokenizer;
use crate::equation_analyzer::utils::{get_x_values, Point};

use rayon::prelude::*;

/// Calculates the result of a mathematical equation.
///
/// # Arguments
/// * `eq` - A string slice containing the equation to calculate
///
/// # Returns
/// * `Ok(f32)` - The numerical result of the calculation
/// * `Err(EquationError)` - An error message, with the character span of the
///   offending input when one exists
///
/// # Examples
/// ```
/// use rusty_maths::equation_analyzer::calculator::calculate;
///
/// let result = calculate("2 + 2 * 3").unwrap();
/// assert_eq!(result, 8.0);
///
/// let err = calculate("2 + foo(3)").unwrap_err();
/// assert_eq!(err.span.map(|s| (s.start, s.end)), Some((4, 7))); // "foo"
/// ```
pub fn calculate(eq: &str) -> Result<f32, EquationError> {
    calculate_with(eq, &Definitions::default())
}

/// Like [`calculate`], with user [`Definitions`] (named values and
/// single-parameter functions) in scope.
///
/// Function bodies resolve late: each call to `calculate_with` compiles
/// them against the definitions as they stand now, so a body referencing
/// `a` sees `a`'s current value. A definition whose body doesn't compile
/// only errors if the equation actually calls it; the error is then tagged
/// with the function's name via [`EquationError::in_function`], and its
/// span refers to the *body* source.
///
/// # Examples
/// ```
/// use rusty_maths::equation_analyzer::calculator::calculate_with;
/// use rusty_maths::equation_analyzer::Definitions;
///
/// let mut defs = Definitions::new();
/// defs.define_value("a", 3.0).unwrap();
/// defs.define_function("g", "a * x^2").unwrap();
///
/// assert_eq!(calculate_with("g(2) + 1", &defs).unwrap(), 13.0);
/// assert_eq!(calculate_with("4 |> g", &defs).unwrap(), 48.0);
///
/// // Redefining `a` changes what `g` computes — late binding.
/// defs.define_value("a", 1.0).unwrap();
/// assert_eq!(calculate_with("g(2)", &defs).unwrap(), 4.0);
/// ```
pub fn calculate_with(eq: &str, defs: &Definitions) -> Result<f32, EquationError> {
    let tokenizer = StreamingTokenizer::new_with(eq, Some(defs))?;
    let parsed = parse(tokenizer)?;
    let ctx = defs.compile();
    evaluate_with(parsed.iter().copied(), None, Some(&ctx))
}

/// Plots a mathematical equation over a range of x values.
///
/// # Arguments
/// * `eq` - A string slice containing the equation to plot (must contain variable 'x')
/// * `x_min` - The minimum x value
/// * `x_max` - The maximum x value
/// * `step_size` - The increment between x values
///
/// # Returns
/// * `Ok(Vec<Point>)` - A vector of points representing the plot
/// * `Err(EquationError)` - An error message, with the character span of the
///   offending input when one exists
///
/// # Examples
/// ```
/// use rusty_maths::equation_analyzer::calculator::plot;
///
/// let points = plot("y = x^2", -2.0, 2.0, 1.0).unwrap();
/// assert_eq!(points.len(), 5); // Points at x = -2, -1, 0, 1, 2
/// ```
pub fn plot(eq: &str, x_min: f32, x_max: f32, step_size: f32) -> Result<Vec<Point>, EquationError> {
    plot_with(eq, x_min, x_max, step_size, &Definitions::default())
}

/// Like [`plot`], with user [`Definitions`] in scope — user-defined
/// functions can be graphed directly.
///
/// # Examples
/// ```
/// use rusty_maths::equation_analyzer::calculator::plot_with;
/// use rusty_maths::equation_analyzer::Definitions;
///
/// let mut defs = Definitions::new();
/// defs.define_function("g", "2x^2").unwrap();
///
/// let points = plot_with("y = g(x)", -1.0, 1.0, 1.0, &defs).unwrap();
/// assert_eq!(points.iter().map(|p| p.y).collect::<Vec<_>>(), vec![2.0, 0.0, 2.0]);
/// ```
pub fn plot_with(
    eq: &str,
    x_min: f32,
    x_max: f32,
    step_size: f32,
    defs: &Definitions,
) -> Result<Vec<Point>, EquationError> {
    // A non-positive step would loop forever below; NaN fails every
    // comparison, so it needs its own check.
    if step_size <= 0.0 || step_size.is_nan() {
        return Err(EquationError::new(format!(
            "Invalid step size {step_size}: step size must be a positive number"
        )));
    }

    let tokenizer = StreamingTokenizer::new_with(eq, Some(defs))?;
    let parsed_eq = parse(tokenizer)?;
    let ctx = defs.compile();

    let x_values = get_x_values(x_min, x_max, step_size);

    let points: Result<Vec<Point>, EquationError> = x_values
        .par_iter()
        .map(|&x| {
            let y = evaluate_with(parsed_eq.iter().copied(), x, Some(&ctx))?;
            Ok(Point { x, y })
        })
        .collect();

    points
}
