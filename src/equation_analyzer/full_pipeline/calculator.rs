use crate::equation_analyzer::vec_pipeline::calculator::Point;
use crate::equation_analyzer::hybrid_pipeline::parser::parse_streaming;
use super::evaluator::evaluate_fully_streaming;
use super::parser::FullyStreamingParser;
use super::tokenizer::StreamingTokenizer;

use rayon::prelude::*;

/// Calculates the result of a mathematical equation using the fully streaming pipeline.
///
/// In this version, the evaluator pulls from the parser, which pulls from the tokenizer.
/// Each stage maintains only minimal buffers (operator stack, value stack, pending tokens).
/// No stage waits for the previous stage to complete before starting.
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
/// use rusty_maths::equation_analyzer::full_pipeline::calculator::calculate;
///
/// let result = calculate("2 + 2 * 3").unwrap();
/// assert_eq!(result, 8.0);
/// ```
///
/// # Architecture
/// ```text
/// StreamingTokenizer → FullyStreamingParser → FullyStreamingEvaluator
///   (pending queue)      (operator stack)        (value stack)
///        ↑                      ↑                      ↑
///        └──────pulls───────────┴──────pulls──────────┘
/// ```
pub fn calculate(eq: &str) -> Result<f32, String> {
    let tokenizer = StreamingTokenizer::new(eq)?;
    let parser = FullyStreamingParser::new(tokenizer);
    evaluate_fully_streaming(parser, None)
}

/// Plots a mathematical equation over a range of x values using the fully streaming pipeline.
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
/// use rusty_maths::equation_analyzer::full_pipeline::calculator::plot;
///
/// let points = plot("y = x^2", -2.0, 2.0, 1.0).unwrap();
/// assert_eq!(points.len(), 5); // Points at x = -2, -1, 0, 1, 2
/// ```
///
/// # Note
/// For plotting, we still parse once then evaluate multiple times for efficiency.
/// The fully streaming approach helps most during the initial parse phase.
pub fn plot(
    eq: &str,
    x_min: f32,
    x_max: f32,
    step_size: f32,
) -> Result<Vec<Point>, String> {
    // For plotting, we parse once and collect RPN tokens
    // Then evaluate in parallel for each x value
    let tokenizer = StreamingTokenizer::new(eq)?;
    let parsed_eq = parse_streaming(tokenizer)?;

    let x_values = get_x_values(x_min, x_max, step_size);

    let points: Result<Vec<Point>, String> = x_values
        .par_iter()
        .map(|&x| {
            // We can't use fully streaming here because we need to reuse the parsed equation
            let y = crate::equation_analyzer::hybrid_pipeline::evaluator::evaluate_streaming(
                &parsed_eq, x,
            )?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_simple_arithmetic() {
        assert_eq!(calculate("2 + 2").unwrap(), 4.0);
        assert_eq!(calculate("10 - 5").unwrap(), 5.0);
        assert_eq!(calculate("3 * 4").unwrap(), 12.0);
    }

    #[test]
    fn test_full_functions() {
        assert!((calculate("sin(0)").unwrap() - 0.0).abs() < 0.0001);
        assert_eq!(calculate("abs(-5)").unwrap(), 5.0);
        assert_eq!(calculate("sqrt(16)").unwrap(), 4.0);
    }

    #[test]
    fn test_full_constants() {
        assert!((calculate("π").unwrap() - std::f32::consts::PI).abs() < 0.0001);
        assert!((calculate("e").unwrap() - std::f32::consts::E).abs() < 0.0001);
    }

    #[test]
    fn test_full_complex() {
        assert_eq!(calculate("(2 + 3) * 4").unwrap(), 20.0);
        assert!((calculate("sqrt(2 ^ 2 + 3 ^ 2)").unwrap() - 3.605551).abs() < 0.001);
    }

    #[test]
    fn test_full_statistical() {
        assert_eq!(calculate("min(1, 2, 3)").unwrap(), 1.0);
        assert_eq!(calculate("max(1, 2, 3)").unwrap(), 3.0);
        assert_eq!(calculate("avg(1, 2, 3, 4, 5)").unwrap(), 3.0);
    }

    #[test]
    fn test_full_plot() {
        let points = plot("y = x^2", -2.0, 2.0, 1.0).unwrap();
        assert_eq!(points.len(), 5);
        assert_eq!(points[2].y, 0.0); // x=0
        assert_eq!(points[4].y, 4.0); // x=2
    }
}
