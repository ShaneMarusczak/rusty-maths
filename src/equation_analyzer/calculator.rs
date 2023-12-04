use crate::equation_analyzer::pipeline::evaluator::evaluate;
use crate::equation_analyzer::pipeline::parser::parse;
use crate::equation_analyzer::pipeline::tokenizer::get_tokens;

use std::sync::Arc;
use std::thread;

pub fn calculate(eq: &str) -> Result<f32, String> {
    evaluate(&parse(get_tokens(eq)?)?)
}

pub fn plot(eq: &str, x_min: f32, x_max: f32, step_size: f32) -> Result<Vec<Point>, String> {
    let eq = Arc::new(eq.to_owned());

    let thread_count = num_cpus::get();
    let mut threads = Vec::with_capacity(thread_count);

    let mut x_values = vec![];

    let mut x_cur = x_min;
    while x_cur <= x_max {
        x_values.push(x_cur);
        x_cur += step_size;
    }
    let chunk_size = (x_values.len() / thread_count) + 1;

    let mut points = Vec::with_capacity(x_values.len());
    let x_chunks: Vec<Vec<_>> = x_values.chunks(chunk_size).map(|s| s.into()).collect();

    for chunk in x_chunks {
        let e = Arc::clone(&eq);
        threads.push(thread::spawn(move || -> Result<Vec<Point>, String> {
            let mut thread_points = Vec::with_capacity(chunk.len());
            for x in chunk {
                let mut prev = ' ';
                let mut expr = String::new();
                for c in e.chars() {
                    if c.eq(&'x') {
                        if prev.is_alphabetic() {
                            expr.push('x');
                        } else if prev.is_ascii_digit() {
                            expr.push('*');
                            expr += &x.to_string();
                        } else {
                            expr += &x.to_string();
                        }
                    } else {
                        expr.push(c);
                    }
                    if !c.is_whitespace() {
                        prev = c;
                    }
                }
                let y = calculate(&expr)?;
                thread_points.push(Point { x, y })
            }
            Ok(thread_points)
        }));
    }
    for thread in threads {
        points.append(&mut thread.join().unwrap()?);
    }

    Ok(points)
}

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }
}
