use crate::equation_analyzer::analyzers::linear_analysis;
use crate::equation_analyzer::analyzers::linear_analysis::detect_linear;
use crate::equation_analyzer::analyzers::quadratic_analysis::{detect_quad, get_abc};
use crate::equation_analyzer::pipeline::evaluator::evaluate;
use crate::equation_analyzer::pipeline::parser::parse;
use crate::equation_analyzer::pipeline::tokenizer::get_tokens;
use crate::utilities::quadratic_eq_f32;

pub fn get_eq_data(
    eq: &str,
    x_min: f32,
    x_max: f32,
    step_size: f32,
) -> Result<EquationData, String> {
    let tokens = get_tokens(eq)?;
    let parsed_eq = parse(tokens)?;

    let mut points = vec![];
    let mut zeros = vec![];
    let literal = eq.to_string();

    if detect_linear(eq) {
        let z = linear_analysis::get_zero(eq);
        if !z.is_nan() {
            zeros.push(z);
        }
    } else if detect_quad(eq) {
        let (a, b, c) = get_abc(eq);

        if let Ok(z) = quadratic_eq_f32(a, b, c) {
            zeros.push(z.0);
            zeros.push(z.1);
        }
    }

    let mut x_cur = x_min;
    while x_cur <= x_max {
        points.push((x_cur, evaluate(&parsed_eq, x_cur)?));
        x_cur += step_size;
    }

    Ok(EquationData {
        literal,
        points,
        zeros,
    })
}

#[derive(Debug, PartialEq)]
pub struct EquationData {
    pub literal: String,
    pub points: Vec<(f32, f32)>,
    pub zeros: Vec<f32>,
}
