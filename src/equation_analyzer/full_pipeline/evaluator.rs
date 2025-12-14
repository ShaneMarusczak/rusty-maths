/// Evaluates a parsed equation in Reverse Polish Notation (RPN) using fully streaming approach.
///
/// This accepts an iterator of tokens (e.g., from FullyStreamingParser) and evaluates them lazily.
///
/// # Arguments
/// * `tokens` - An iterator of tokens in RPN format
/// * `x` - Optional value of the variable x (defaults to 0.0 if None)
///
/// # Returns
/// * `Ok(f32)` - The result of the evaluation
/// * `Err(String)` - An error message if evaluation fails
pub(crate) fn evaluate_fully_streaming<I>(
    tokens: I,
    x: impl Into<Option<f32>>,
) -> Result<f32, String>
where
    I: IntoIterator<Item = Result<crate::equation_analyzer::structs::token::Token, String>>,
{
    // Extract tokens from Results
    let unwrapped_tokens = tokens.into_iter().collect::<Result<Vec<_>, _>>()?;
    crate::equation_analyzer::core::evaluator::evaluate(unwrapped_tokens, x)
}
