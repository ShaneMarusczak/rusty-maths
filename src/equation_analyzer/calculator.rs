use crate::equation_analyzer::pipeline::evaluator::evaluate;
use crate::equation_analyzer::pipeline::parser::parse;
use crate::equation_analyzer::pipeline::tokenizer::get_tokens;

pub fn calculate(eq: &str) -> Result<f32, String> {
    let tokens = get_tokens(eq)?;
    let parsed_eq = parse(tokens)?;
    evaluate(&parsed_eq, 0.0)
}
