use crate::equation_analyzer::calculator::Point;
use crate::equation_analyzer::pipeline::fully_streaming_evaluator::evaluate_fully_streaming;
use crate::equation_analyzer::pipeline::fully_streaming_parser::FullyStreamingParser;
use crate::equation_analyzer::pipeline::streaming_parser::parse_streaming;
use crate::equation_analyzer::pipeline::streaming_tokenizer::StreamingTokenizer;

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
/// use rusty_maths::equation_analyzer::fully_streaming_calculator::fully_streaming_calculate;
///
/// let result = fully_streaming_calculate("2 + 2 * 3").unwrap();
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
pub fn fully_streaming_calculate(eq: &str) -> Result<f32, String> {
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
/// use rusty_maths::equation_analyzer::fully_streaming_calculator::fully_streaming_plot;
///
/// let points = fully_streaming_plot("y = x^2", -2.0, 2.0, 1.0).unwrap();
/// assert_eq!(points.len(), 5); // Points at x = -2, -1, 0, 1, 2
/// ```
///
/// # Note
/// For plotting, we still parse once then evaluate multiple times for efficiency.
/// The fully streaming approach helps most during the initial parse phase.
pub fn fully_streaming_plot(
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
            let y = crate::equation_analyzer::pipeline::streaming_evaluator::evaluate_streaming(
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
    use crate::equation_analyzer::calculator::calculate;
    use crate::equation_analyzer::streaming_calculator::streaming_calculate;

    #[test]
    fn test_fully_streaming_vs_original() {
        let equations = vec![
            "2 + 2",
            "sin(π / 2)",
            "sqrt(2 ^ 2 + 3 ^ 2)",
            "3! + 4!",
            "min(1, 2, 3)",
            "avg(1, 2, 3, 4, 5)",
        ];

        for eq in equations {
            let original = calculate(eq).unwrap();
            let fully_streaming = fully_streaming_calculate(eq).unwrap();
            assert!(
                (original - fully_streaming).abs() < 0.0001,
                "Results differ for equation: {}. Original: {}, Fully Streaming: {}",
                eq,
                original,
                fully_streaming
            );
        }
    }

    #[test]
    fn test_three_way_comparison() {
        let equations = vec![
            "2 + 3 * 4",
            "(2 + 3) * 4",
            "log_10(100)",
            "abs(-10) + abs(-5)",
        ];

        for eq in equations {
            let vec_result = calculate(eq).unwrap();
            let streaming_result = streaming_calculate(eq).unwrap();
            let fully_streaming_result = fully_streaming_calculate(eq).unwrap();

            assert!(
                (vec_result - streaming_result).abs() < 0.0001,
                "Vec vs Streaming differ for: {}",
                eq
            );
            assert!(
                (vec_result - fully_streaming_result).abs() < 0.0001,
                "Vec vs Fully Streaming differ for: {}",
                eq
            );
            assert!(
                (streaming_result - fully_streaming_result).abs() < 0.0001,
                "Streaming vs Fully Streaming differ for: {}",
                eq
            );
        }
    }

    #[test]
    #[ignore]
    fn benchmark_all_three_approaches() {
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

        // Vec-based
        let start = Instant::now();
        for _ in 0..ITERATIONS {
            for eq in &equations {
                let _ = calculate(eq).unwrap();
            }
        }
        let vec_duration = start.elapsed();

        // Hybrid streaming
        let start = Instant::now();
        for _ in 0..ITERATIONS {
            for eq in &equations {
                let _ = streaming_calculate(eq).unwrap();
            }
        }
        let streaming_duration = start.elapsed();

        // Fully streaming
        let start = Instant::now();
        for _ in 0..ITERATIONS {
            for eq in &equations {
                let _ = fully_streaming_calculate(eq).unwrap();
            }
        }
        let fully_streaming_duration = start.elapsed();

        println!("\n=== Three-Way Benchmark ({} iterations) ===", ITERATIONS);
        println!("Vec-based:         {:?}", vec_duration);
        println!("Hybrid Streaming:  {:?}", streaming_duration);
        println!("Fully Streaming:   {:?}", fully_streaming_duration);
        println!("\nSpeedup vs Vec:");
        println!(
            "  Hybrid:  {:.2}x",
            vec_duration.as_secs_f64() / streaming_duration.as_secs_f64()
        );
        println!(
            "  Full:    {:.2}x",
            vec_duration.as_secs_f64() / fully_streaming_duration.as_secs_f64()
        );
        println!("\nFully Streaming vs Hybrid Streaming:");
        println!(
            "  {:.2}x",
            streaming_duration.as_secs_f64() / fully_streaming_duration.as_secs_f64()
        );
    }
}
