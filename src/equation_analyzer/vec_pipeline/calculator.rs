use super::evaluator::evaluate;
use super::parser::parse;
use super::tokenizer::get_tokens;

use rayon::prelude::*;

/// Calculates the result of a mathematical equation.
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
/// use rusty_maths::equation_analyzer::vec_pipeline::calculator::calculate;
///
/// let result = calculate("2 + 2 * 3").unwrap();
/// assert_eq!(result, 8.0);
/// ```
pub fn calculate(eq: &str) -> Result<f32, String> {
    evaluate(&parse(get_tokens(eq)?)?, None)
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
/// * `Err(String)` - An error message if the equation is invalid or plotting fails
///
/// # Examples
/// ```
/// use rusty_maths::equation_analyzer::vec_pipeline::calculator::plot;
///
/// let points = plot("y = x^2", -2.0, 2.0, 1.0).unwrap();
/// assert_eq!(points.len(), 5); // Points at x = -2, -1, 0, 1, 2
/// ```
pub fn plot(eq: &str, x_min: f32, x_max: f32, step_size: f32) -> Result<Vec<Point>, String> {
    let tokens = get_tokens(eq)?;
    let parsed_eq = parse(tokens)?;

    let x_values = get_x_values(x_min, x_max, step_size);

    let points: Result<Vec<Point>, String> = x_values
        .par_iter()
        .map(|&x| {
            let y = evaluate(&parsed_eq, x)?;
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

/// Represents a point in 2D space for plotting equations.
#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    /// The x-coordinate
    pub x: f32,
    /// The y-coordinate (result of evaluating the equation at x)
    pub y: f32,
}

impl Point {
    /// Creates a new Point with the given coordinates.
    ///
    /// # Arguments
    /// * `x` - The x-coordinate
    /// * `y` - The y-coordinate
    ///
    /// # Returns
    /// A new Point instance
    pub fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_simple_arithmetic() {
        assert_eq!(calculate("2 + 2").unwrap(), 4.0);
        assert_eq!(calculate("10 - 5").unwrap(), 5.0);
        assert_eq!(calculate("3 * 4").unwrap(), 12.0);
    }

    #[test]
    fn test_vec_functions() {
        assert!((calculate("sin(0)").unwrap() - 0.0).abs() < 0.0001);
        assert_eq!(calculate("abs(-5)").unwrap(), 5.0);
        assert_eq!(calculate("sqrt(16)").unwrap(), 4.0);
    }

    #[test]
    fn test_vec_constants() {
        assert!((calculate("π").unwrap() - std::f32::consts::PI).abs() < 0.0001);
        assert!((calculate("e").unwrap() - std::f32::consts::E).abs() < 0.0001);
    }

    #[test]
    fn test_vec_complex() {
        assert_eq!(calculate("(2 + 3) * 4").unwrap(), 20.0);
        assert!((calculate("sqrt(2 ^ 2 + 3 ^ 2)").unwrap() - 3.605551).abs() < 0.001);
    }

    #[test]
    fn test_vec_statistical() {
        assert_eq!(calculate("min(1, 2, 3)").unwrap(), 1.0);
        assert_eq!(calculate("max(1, 2, 3)").unwrap(), 3.0);
        assert_eq!(calculate("avg(1, 2, 3, 4, 5)").unwrap(), 3.0);
    }

    #[test]
    fn test_vec_plot() {
        let points = plot("y = x^2", -2.0, 2.0, 1.0).unwrap();
        assert_eq!(points.len(), 5);
        assert_eq!(points[2].y, 0.0); // x=0
        assert_eq!(points[4].y, 4.0); // x=2
    }
}
