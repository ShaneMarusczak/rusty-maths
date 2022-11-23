use crate::equation_analyzer::pipeline::evaluator::evaluate;
use crate::equation_analyzer::pipeline::parser::parse;
use crate::equation_analyzer::pipeline::tokenizer::get_tokens;

use super::eq_data_builder::get_eq_data;

pub fn calculate(eq: &str) -> Result<f32, String> {
    let tokens = get_tokens(eq)?;
    let parsed_eq = parse(tokens)?;
    evaluate(&parsed_eq, 0.0)
}

pub fn plot(eq: &str, x_min: f32, x_max: f32, step_size: f32) -> Result<Vec<(f32, f32)>, String> {
    let eq_data = get_eq_data(eq, x_min, x_max, step_size);

    return if eq_data.is_err() {
        Err(eq_data.unwrap_err())
    } else {
        Ok(eq_data.unwrap().points)
    };
}
