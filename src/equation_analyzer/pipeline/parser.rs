use crate::equation_analyzer::structs::token::Token;

/// Parses a Vec of tokens into Reverse Polish Notation (RPN).
///
/// This is a wrapper around the core parser that handles Vec<Token> input.
///
/// # Arguments
/// * `tokens` - A vector of tokens in infix notation
///
/// # Returns
/// * `Ok(Vec<Token>)` - The tokens in RPN format
/// * `Err(String)` - An error message if parsing fails
pub(crate) fn parse(tokens: Vec<Token>) -> Result<Vec<Token>, String> {
    // Convert Vec<Token> to Iterator<Item = Result<Token, String>>
    let token_results = tokens.into_iter().map(Ok);
    crate::equation_analyzer::core::parser::parse(token_results)
}
