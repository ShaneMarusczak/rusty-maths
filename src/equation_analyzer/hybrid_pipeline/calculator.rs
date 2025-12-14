use crate::equation_analyzer::vec_pipeline::calculator::Point;
use super::evaluator::evaluate_streaming;
use super::parser::parse_streaming;
use super::tokenizer::StreamingTokenizer;

use rayon::prelude::*;

/// Calculates the result of a mathematical equation using the hybrid streaming pipeline.
///
/// # Arguments
/// * `eq` - A string slice containing the equation to calculate
///
/// # Returns
/// * `Ok(f32)` - The numerical result of the calculation
/// * `Err(String)` - An error message if the equation is invalid or calculation fails
///
/// # Examples
/// ```
/// use rusty_maths::equation_analyzer::hybrid_pipeline::calculator::calculate;
///
/// let result = calculate("2 + 2 * 3").unwrap();
/// assert_eq!(result, 8.0);
/// ```
///
/// # Note
/// This is the hybrid streaming implementation that uses an iterator-based tokenizer
/// for more efficient memory usage and lazy evaluation.
pub fn calculate(eq: &str) -> Result<f32, String> {
    let tokenizer = StreamingTokenizer::new(eq)?;
    let parsed = parse_streaming(tokenizer)?;
    evaluate_streaming(&parsed, None)
}

/// Plots a mathematical equation over a range of x values using the hybrid streaming pipeline.
///
/// # Arguments
/// * `eq` - A string slice containing the equation to plot (must contain variable 'x')
/// * `x_min` - The minimum x value
/// * `x_max` - The maximum x value
/// * `step_size` - The increment between x values
///
/// # Returns
/// * `Ok(Vec<Point>)` - A vector of points representing the plot
/// * `Err(String)` - An error message if the equation is invalid or plotting fails
///
/// # Examples
/// ```
/// use rusty_maths::equation_analyzer::hybrid_pipeline::calculator::plot;
///
/// let points = plot("y = x^2", -2.0, 2.0, 1.0).unwrap();
/// assert_eq!(points.len(), 5); // Points at x = -2, -1, 0, 1, 2
/// ```
///
/// # Note
/// This is the hybrid streaming implementation that uses an iterator-based tokenizer.
/// The equation is still parsed once and then evaluated multiple times for efficiency.
pub fn plot(eq: &str, x_min: f32, x_max: f32, step_size: f32) -> Result<Vec<Point>, String> {
    let tokenizer = StreamingTokenizer::new(eq)?;
    let parsed_eq = parse_streaming(tokenizer)?;

    let x_values = get_x_values(x_min, x_max, step_size);

    let points: Result<Vec<Point>, String> = x_values
        .par_iter()
        .map(|&x| {
            let y = evaluate_streaming(&parsed_eq, x)?;
            Ok(Point { x, y })
        })
        .collect();

    points
}

fn get_x_values(x_min: f32, x_max: f32, step_size: f32) -> Vec<f32> {
    let x_range = ((x_max - x_min) / step_size).ceil() as usize + 1;
    let mut x_values = Vec::with_capacity(x_range);

    let mut x_cur = x_min;
    while x_cur <= x_max {
        x_values.push(x_cur);
        x_cur += step_size;
    }
    x_values
}
