use crate::equation_analyzer::pipeline::evaluator::evaluate;
use crate::equation_analyzer::pipeline::parser::parse;
use crate::equation_analyzer::pipeline::tokenizer::get_tokens;

use std::sync::Arc;
use std::thread;

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
/// use rusty_maths::equation_analyzer::calculator::calculate;
///
/// let result = calculate("2 + 2 * 3").unwrap();
/// assert_eq!(result, 8.0);
/// ```
pub fn calculate(eq: &str) -> Result<f32, String> {
    evaluate(&parse(get_tokens(eq)?)?, 0.0)
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
/// use rusty_maths::equation_analyzer::calculator::plot;
///
/// let points = plot("y = x^2", -2.0, 2.0, 1.0).unwrap();
/// assert_eq!(points.len(), 5); // Points at x = -2, -1, 0, 1, 2
/// ```
pub fn plot(eq: &str, x_min: f32, x_max: f32, step_size: f32) -> Result<Vec<Point>, String> {
    let tokens = get_tokens(eq)?;
    let parsed_eq = Arc::new(parse(tokens)?);

    let x_values = get_x_values(x_min, x_max, step_size);
    let mut points = Vec::with_capacity(x_values.len());

    if x_values.len() > 500 {
        let thread_count = num_cpus::get();
        let chunk_size = (x_values.len() / thread_count) + 1;
        let mut threads = Vec::with_capacity(thread_count);
        let x_chunks: Vec<Vec<f32>> = x_values.chunks(chunk_size).map(|s| s.into()).collect();

        for chunk in x_chunks {
            let parsed_eq_clone = Arc::clone(&parsed_eq);
            threads.push(thread::spawn(move || -> Result<Vec<Point>, String> {
                let mut thread_points = Vec::with_capacity(chunk.len());
                for x in chunk {
                    let y = evaluate(&parsed_eq_clone, x)?;
                    thread_points.push(Point { x, y })
                }
                Ok(thread_points)
            }));
        }
        for thread in threads {
            points.append(&mut thread.join().unwrap()?);
        }
    } else {
        for x in x_values {
            let y = evaluate(&parsed_eq, x)?;
            points.push(Point { x, y })
        }
    }
    Ok(points)
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
