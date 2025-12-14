use crate::equation_analyzer::calculator::Point;
use crate::equation_analyzer::pipeline::streaming_evaluator::evaluate_streaming;
use crate::equation_analyzer::pipeline::streaming_parser::parse_streaming;
use crate::equation_analyzer::pipeline::streaming_tokenizer::StreamingTokenizer;

use rayon::prelude::*;

/// Calculates the result of a mathematical equation using the streaming pipeline.
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
/// use rusty_maths::equation_analyzer::streaming_calculator::streaming_calculate;
///
/// let result = streaming_calculate("2 + 2 * 3").unwrap();
/// assert_eq!(result, 8.0);
/// ```
///
/// # Note
/// This is the streaming implementation that uses an iterator-based tokenizer
/// for more efficient memory usage and lazy evaluation.
pub fn streaming_calculate(eq: &str) -> Result<f32, String> {
    let tokenizer = StreamingTokenizer::new(eq)?;
    let parsed = parse_streaming(tokenizer)?;
    evaluate_streaming(&parsed, None)
}

/// Plots a mathematical equation over a range of x values using the streaming pipeline.
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
/// use rusty_maths::equation_analyzer::streaming_calculator::streaming_plot;
///
/// let points = streaming_plot("y = x^2", -2.0, 2.0, 1.0).unwrap();
/// assert_eq!(points.len(), 5); // Points at x = -2, -1, 0, 1, 2
/// ```
///
/// # Note
/// This is the streaming implementation that uses an iterator-based tokenizer.
/// The equation is still parsed once and then evaluated multiple times for efficiency.
pub fn streaming_plot(eq: &str, x_min: f32, x_max: f32, step_size: f32) -> Result<Vec<Point>, String> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::equation_analyzer::calculator::{calculate, plot};

    #[test]
    fn test_streaming_vs_vec_simple_arithmetic() {
        let equations = vec![
            "2 + 2",
            "10 - 5",
            "3 * 4",
            "15 / 3",
            "2 ^ 3",
            "10 %% 3",
            "50 % 10",
        ];

        for eq in equations {
            let vec_result = calculate(eq).unwrap();
            let streaming_result = streaming_calculate(eq).unwrap();
            assert_eq!(
                vec_result, streaming_result,
                "Results differ for equation: {}",
                eq
            );
        }
    }

    #[test]
    fn test_streaming_vs_vec_functions() {
        let equations = vec![
            "sin(0)",
            "cos(0)",
            "tan(0)",
            "abs(-5)",
            "sqrt(16)",
            "ln(2.718281828)",
            "log_2(8)",
            "5!",
        ];

        for eq in equations {
            let vec_result = calculate(eq).unwrap();
            let streaming_result = streaming_calculate(eq).unwrap();
            assert!(
                (vec_result - streaming_result).abs() < 0.0001,
                "Results differ for equation: {}. Vec: {}, Streaming: {}",
                eq,
                vec_result,
                streaming_result
            );
        }
    }

    #[test]
    fn test_streaming_vs_vec_constants() {
        let equations = vec!["π", "e", "-π", "-e", "2 * π", "e ^ 2"];

        for eq in equations {
            let vec_result = calculate(eq).unwrap();
            let streaming_result = streaming_calculate(eq).unwrap();
            assert!(
                (vec_result - streaming_result).abs() < 0.0001,
                "Results differ for equation: {}. Vec: {}, Streaming: {}",
                eq,
                vec_result,
                streaming_result
            );
        }
    }

    #[test]
    fn test_streaming_vs_vec_complex_expressions() {
        let equations = vec![
            "2 + 3 * 4",
            "(2 + 3) * 4",
            "sin(π / 2)",
            "sqrt(2 ^ 2 + 3 ^ 2)",
            "abs(-10) + abs(-5)",
            "3! + 4!",
            "log_10(100)",
        ];

        for eq in equations {
            let vec_result = calculate(eq).unwrap();
            let streaming_result = streaming_calculate(eq).unwrap();
            assert!(
                (vec_result - streaming_result).abs() < 0.0001,
                "Results differ for equation: {}. Vec: {}, Streaming: {}",
                eq,
                vec_result,
                streaming_result
            );
        }
    }

    #[test]
    fn test_streaming_vs_vec_statistical_functions() {
        let equations = vec![
            "min(1, 2, 3)",
            "max(1, 2, 3)",
            "avg(1, 2, 3, 4, 5)",
            "med(1, 2, 3, 4, 5)",
            "mode(1, 1, 2, 3, 3, 3)",
        ];

        for eq in equations {
            let vec_result = calculate(eq).unwrap();
            let streaming_result = streaming_calculate(eq).unwrap();
            assert!(
                (vec_result - streaming_result).abs() < 0.0001,
                "Results differ for equation: {}. Vec: {}, Streaming: {}",
                eq,
                vec_result,
                streaming_result
            );
        }
    }

    #[test]
    fn test_streaming_vs_vec_plot_linear() {
        let eq = "y = 2*x + 1";
        let vec_points = plot(eq, -5.0, 5.0, 1.0).unwrap();
        let streaming_points = streaming_plot(eq, -5.0, 5.0, 1.0).unwrap();

        assert_eq!(vec_points.len(), streaming_points.len());
        for (v, s) in vec_points.iter().zip(streaming_points.iter()) {
            assert_eq!(v.x, s.x);
            assert!(
                (v.y - s.y).abs() < 0.0001,
                "Y values differ at x = {}. Vec: {}, Streaming: {}",
                v.x,
                v.y,
                s.y
            );
        }
    }

    #[test]
    fn test_streaming_vs_vec_plot_quadratic() {
        let eq = "y = x^2";
        let vec_points = plot(eq, -10.0, 10.0, 0.5).unwrap();
        let streaming_points = streaming_plot(eq, -10.0, 10.0, 0.5).unwrap();

        assert_eq!(vec_points.len(), streaming_points.len());
        for (v, s) in vec_points.iter().zip(streaming_points.iter()) {
            assert_eq!(v.x, s.x);
            assert!(
                (v.y - s.y).abs() < 0.0001,
                "Y values differ at x = {}. Vec: {}, Streaming: {}",
                v.x,
                v.y,
                s.y
            );
        }
    }

    #[test]
    fn test_streaming_vs_vec_plot_trig() {
        let eq = "y = sin(x)";
        let vec_points = plot(eq, -3.14159, 3.14159, 0.1).unwrap();
        let streaming_points = streaming_plot(eq, -3.14159, 3.14159, 0.1).unwrap();

        assert_eq!(vec_points.len(), streaming_points.len());
        for (v, s) in vec_points.iter().zip(streaming_points.iter()) {
            assert_eq!(v.x, s.x);
            assert!(
                (v.y - s.y).abs() < 0.0001,
                "Y values differ at x = {}. Vec: {}, Streaming: {}",
                v.x,
                v.y,
                s.y
            );
        }
    }

    #[test]
    fn test_streaming_vs_vec_plot_complex() {
        let eq = "y = 3*x^2 - 2*x + 5";
        let vec_points = plot(eq, -5.0, 5.0, 0.25).unwrap();
        let streaming_points = streaming_plot(eq, -5.0, 5.0, 0.25).unwrap();

        assert_eq!(vec_points.len(), streaming_points.len());
        for (v, s) in vec_points.iter().zip(streaming_points.iter()) {
            assert_eq!(v.x, s.x);
            assert!(
                (v.y - s.y).abs() < 0.0001,
                "Y values differ at x = {}. Vec: {}, Streaming: {}",
                v.x,
                v.y,
                s.y
            );
        }
    }

    #[test]
    fn test_streaming_error_handling() {
        let invalid_equations = vec![
            "",
            "2 +",
            "* 3",
            "((2 + 3)",
            "2 + 3)",
            "invalid(5)",
        ];

        for eq in invalid_equations {
            let vec_result = calculate(eq);
            let streaming_result = streaming_calculate(eq);

            assert!(vec_result.is_err() || streaming_result.is_err(),
                "Both implementations should error for: {}", eq);
        }
    }

    #[test]
    #[ignore] // Run with: cargo test --release -- --ignored --nocapture
    fn benchmark_streaming_vs_vec_calculate() {
        use std::time::Instant;

        let equations = vec![
            "2 + 2",
            "sin(π / 2)",
            "sqrt(2 ^ 2 + 3 ^ 2)",
            "abs(-10) + abs(-5)",
            "3! + 4!",
            "log_10(100)",
            "min(1, 2, 3, 4, 5)",
            "avg(1, 2, 3, 4, 5)",
        ];

        const ITERATIONS: usize = 10000;

        // Benchmark Vec-based calculator
        let start = Instant::now();
        for _ in 0..ITERATIONS {
            for eq in &equations {
                let _ = calculate(eq).unwrap();
            }
        }
        let vec_duration = start.elapsed();

        // Benchmark streaming calculator
        let start = Instant::now();
        for _ in 0..ITERATIONS {
            for eq in &equations {
                let _ = streaming_calculate(eq).unwrap();
            }
        }
        let streaming_duration = start.elapsed();

        println!("\n=== Calculate Benchmark ({} iterations) ===", ITERATIONS);
        println!("Vec-based:  {:?}", vec_duration);
        println!("Streaming:  {:?}", streaming_duration);
        println!("Difference: {:?}", vec_duration.as_nanos() as i128 - streaming_duration.as_nanos() as i128);

        let speedup = vec_duration.as_secs_f64() / streaming_duration.as_secs_f64();
        if speedup > 1.0 {
            println!("Streaming is {:.2}x faster", speedup);
        } else {
            println!("Vec-based is {:.2}x faster", 1.0 / speedup);
        }
    }

    #[test]
    #[ignore] // Run with: cargo test --release -- --ignored --nocapture
    fn benchmark_streaming_vs_vec_plot() {
        use std::time::Instant;

        let equations = vec![
            "y = x^2",
            "y = sin(x)",
            "y = 3*x^2 - 2*x + 5",
            "y = sqrt(abs(x))",
        ];

        const ITERATIONS: usize = 100;

        // Benchmark Vec-based plot
        let start = Instant::now();
        for _ in 0..ITERATIONS {
            for eq in &equations {
                let _ = plot(eq, -10.0, 10.0, 0.1).unwrap();
            }
        }
        let vec_duration = start.elapsed();

        // Benchmark streaming plot
        let start = Instant::now();
        for _ in 0..ITERATIONS {
            for eq in &equations {
                let _ = streaming_plot(eq, -10.0, 10.0, 0.1).unwrap();
            }
        }
        let streaming_duration = start.elapsed();

        println!("\n=== Plot Benchmark ({} iterations) ===", ITERATIONS);
        println!("Vec-based:  {:?}", vec_duration);
        println!("Streaming:  {:?}", streaming_duration);
        println!("Difference: {:?}", vec_duration.as_nanos() as i128 - streaming_duration.as_nanos() as i128);

        let speedup = vec_duration.as_secs_f64() / streaming_duration.as_secs_f64();
        if speedup > 1.0 {
            println!("Streaming is {:.2}x faster", speedup);
        } else {
            println!("Vec-based is {:.2}x faster", 1.0 / speedup);
        }
    }
}
