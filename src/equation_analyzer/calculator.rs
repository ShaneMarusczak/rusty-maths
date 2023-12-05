use crate::equation_analyzer::pipeline::evaluator::evaluate;
use crate::equation_analyzer::pipeline::parser::parse;
use crate::equation_analyzer::pipeline::tokenizer::get_tokens;

use std::sync::Arc;
use std::thread;

pub fn calculate(eq: &str) -> Result<f32, String> {
    evaluate(&parse(get_tokens(eq)?)?)
}

pub fn plot(eq: &str, x_min: f32, x_max: f32, step_size: f32) -> Result<Vec<Point>, String> {
    let x_values = get_x_values(x_min, x_max, step_size);

    let mut points = Vec::with_capacity(x_values.len());

    let preprocessed = preprocess(eq);

    if x_values.len() > 500 {
        let thread_count = num_cpus::get();

        let chunk_size = (x_values.len() / thread_count) + 1;
        let mut threads = Vec::with_capacity(thread_count);
        let x_chunks: Vec<Vec<f32>> = x_values.chunks(chunk_size).map(|s| s.into()).collect();

        let expr_arc = Arc::new(preprocessed);

        for chunk in x_chunks {
            let expr_t = Arc::clone(&expr_arc);
            threads.push(thread::spawn(move || -> Result<Vec<Point>, String> {
                let mut thread_points = Vec::with_capacity(chunk.len());
                for x in chunk {
                    let val = expr_t.replace("[x]", &x.to_string());
                    let y = calculate(&val)?;
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
            let val = preprocessed.replace("[x]", &x.to_string());
            let y = calculate(&val)?;
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

fn preprocess(e: &str) -> String {
    let mut prev = ' ';
    let mut expr = String::new();
    let mut paren_count = 0;
    for c in e.chars() {
        if c.eq(&'^') {
            //3^3^3*2 -> 3^(3^(3*2))
            expr.push('^');
            expr.push('(');
            paren_count += 1;
        } else if c.eq(&'x') {
            //prev is the immediately prior char, whitespace included
            if prev.is_alphabetic() {
                //max -> max
                expr.push('x');
            } else if prev.is_ascii_digit() {
                //2x -> 2*[x]
                //2 x is invalid
                //no spaces
                expr.push('*');
                expr.push('[');
                expr.push('x');
                expr.push(']');
            } else {
                //2+x -> 2+[x]
                expr.push('[');
                expr.push('x');
                expr.push(']');
            }
        } else {
            expr.push(c);
        }
        if c.is_whitespace() {
            for _ in 0..paren_count {
                expr.push(')');
            }
            paren_count = 0;
        }
        prev = c;
    }
    for _ in 0..paren_count {
        expr.push(')');
    }
    expr
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
