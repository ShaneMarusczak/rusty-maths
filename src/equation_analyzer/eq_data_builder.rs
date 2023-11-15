use crate::equation_analyzer::pipeline::evaluator::evaluate;
use crate::equation_analyzer::pipeline::parser::parse;
use crate::equation_analyzer::pipeline::tokenizer::get_tokens;

use std::sync::Arc;
use std::thread;

pub fn get_eq_data(
    eq: &str,
    x_min: f32,
    x_max: f32,
    step_size: f32,
) -> Result<EquationData, String> {
    let tokens = get_tokens(eq)?;

    let parsed_eq = Arc::new(parse(tokens)?);

    let mut threads = vec![];

    let mut x_values = vec![];

    let mut x_cur = x_min;
    while x_cur <= x_max {
        x_values.push(x_cur);
        x_cur += step_size;
    }

    let thread_count = num_cpus::get();

    let mut points = Vec::with_capacity(x_values.len());

    let chunk_size = (x_values.len() / thread_count) + 1;

    let x_chunks: Vec<Vec<_>> = x_values.chunks(chunk_size).map(|s| s.into()).collect();

    assert!(thread_count >= x_chunks.len());

    for chunk in x_chunks {
        let parsed_eq = Arc::clone(&parsed_eq);

        threads.push(thread::spawn(move || {
            let mut thread_points = Vec::with_capacity(chunk.len());
            for x in chunk {
                let result = (x, evaluate(&parsed_eq, x).expect("evaluation failed"));
                thread_points.push(result);
            }
            thread_points
        }));
    }
    for thread in threads {
        points.append(&mut thread.join().unwrap());
    }

    Ok(EquationData {
        literal: eq.to_string(),
        points,
    })
}

#[derive(Debug, PartialEq)]
pub struct EquationData {
    pub literal: String,
    pub points: Vec<(f32, f32)>,
}
