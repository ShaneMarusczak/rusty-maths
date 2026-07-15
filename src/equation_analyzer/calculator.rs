use crate::equation_analyzer::errors::EquationError;
use crate::equation_analyzer::pipeline::evaluator::evaluate;
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
    let tokenizer = StreamingTokenizer::new(eq)?;
    let parsed = parse(tokenizer)?;
    evaluate(parsed.iter().copied(), None)
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
    // A non-positive step would loop forever below; NaN fails every
    // comparison, so it needs its own check.
    if step_size <= 0.0 || step_size.is_nan() {
        return Err(EquationError::new(format!(
            "Invalid step size {step_size}: step size must be a positive number"
        )));
    }

    let tokenizer = StreamingTokenizer::new(eq)?;
    let parsed_eq = parse(tokenizer)?;

    let x_values = get_x_values(x_min, x_max, step_size);

    let points: Result<Vec<Point>, EquationError> = x_values
        .par_iter()
        .map(|&x| {
            let y = evaluate(parsed_eq.iter().copied(), x)?;
            Ok(Point { x, y })
        })
        .collect();

    points
}
