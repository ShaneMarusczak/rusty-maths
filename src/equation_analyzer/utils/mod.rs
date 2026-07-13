/// Shared utilities for the equation analyzer pipeline.
///
/// Represents a point in 2D space for plotting equations.
///
/// This struct is used by all pipeline calculators to return plot results.
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

/// Generates x values for plotting based on range and step size
///
/// # Arguments
/// * `x_min` - Minimum x value
/// * `x_max` - Maximum x value
/// * `step_size` - Step size between x values
///
/// # Returns
/// Vector of x values from x_min to x_max (inclusive) with the given step size
pub fn get_x_values(x_min: f32, x_max: f32, step_size: f32) -> Vec<f32> {
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
    fn test_get_x_values() {
        let values = get_x_values(-2.0, 2.0, 1.0);
        assert_eq!(values.len(), 5);
        assert_eq!(values, vec![-2.0, -1.0, 0.0, 1.0, 2.0]);
    }

    #[test]
    fn test_get_x_values_fractional_step() {
        let values = get_x_values(0.0, 1.0, 0.25);
        assert_eq!(values.len(), 5);
        assert_eq!(values, vec![0.0, 0.25, 0.5, 0.75, 1.0]);
    }
}
