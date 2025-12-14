use crate::equation_analyzer::structs::operands::{get_operator, Assoc, Operand};
use crate::equation_analyzer::structs::token::{ParamToken, Token, TokenType};
use crate::equation_analyzer::utils::make_synthetic_token;

/// Generic Shunting Yard parser that works with any iterator of tokens.
///
/// This is the core parsing logic shared by all pipeline implementations.
/// Converts infix notation to Reverse Polish Notation (RPN) using Dijkstra's Shunting Yard algorithm.
///
/// # Arguments
/// * `tokens` - An iterator yielding tokens in infix notation (can be Result<Token, String>)
///
/// # Returns
/// * `Ok(Vec<Token>)` - The tokens in RPN format
/// * `Err(String)` - An error message if parsing fails
///
/// # Algorithm
/// Uses Dijkstra's Shunting Yard algorithm to convert infix notation to RPN:
/// 1. Maintains an operator stack for pending operators
/// 2. Builds output queue of tokens in RPN order
/// 3. Handles operator precedence and associativity
/// 4. Manages parentheses and function calls
/// 5. Special handling for variadic functions (avg, min, max, etc.)
///
/// # Note
/// This algorithm requires buffering the output due to the nature of infix-to-RPN conversion.
/// The output is always a Vec<Token>, but the input can be any iterator (streaming or not).
pub(crate) fn parse<I>(tokens: I) -> Result<Vec<Token>, String>
where
    I: IntoIterator<Item = Result<Token, String>>,
{
    let mut operator_stack: Vec<Operand> = Vec::new();
    let mut output: Vec<Token> = Vec::new();
    let mut paren_depth = 0;
    let mut param_token = ParamToken::None;
    let mut found_end = false;

    for token_result in tokens {
        let token = token_result?;

        // Handle variadic function parameter collection
        if param_token != ParamToken::None {
            if matches!(token.token_type, TokenType::Number | TokenType::X) {
                output.push(token);
                continue;
            } else if token.token_type == TokenType::Comma {
                continue;
            } else if token.token_type == TokenType::CloseParen {
                output.push(make_synthetic_token(param_token.to_end_token_type()));
                param_token = ParamToken::None;
                continue;
            } else {
                return Err("Params can only be numbers".to_string());
            }
        }

        match token.token_type {
            // Skip equation markers and commas
            TokenType::Y | TokenType::Equal | TokenType::Comma => continue,

            // Constants go directly to output
            TokenType::_Pi | TokenType::_E | TokenType::NegPi | TokenType::NegE => {
                output.push(token);
            }

            // Variadic functions - start parameter collection
            TokenType::Avg => {
                output.push(token);
                param_token = ParamToken::Avg;
            }

            TokenType::Choice => {
                output.push(token);
                param_token = ParamToken::Choice;
            }

            TokenType::Min => {
                output.push(token);
                param_token = ParamToken::Min;
            }

            TokenType::Max => {
                output.push(token);
                param_token = ParamToken::Max;
            }

            TokenType::Med => {
                output.push(token);
                param_token = ParamToken::Med;
            }

            TokenType::Mode => {
                output.push(token);
                param_token = ParamToken::Mode;
            }

            // Functions and opening parenthesis go on operator stack
            TokenType::Sin
            | TokenType::Cos
            | TokenType::Tan
            | TokenType::Asin
            | TokenType::Acos
            | TokenType::Atan
            | TokenType::Abs
            | TokenType::Sqrt
            | TokenType::Ln
            | TokenType::Log
            | TokenType::OpenParen => {
                paren_depth += 1;
                operator_stack.push(get_operator(token));
            }

            // Closing parenthesis: pop operators until matching open paren
            TokenType::CloseParen => {
                paren_depth -= 1;

                if paren_depth < 0 {
                    return Err("Invalid closing parenthesis".to_string());
                }

                // Pop operators until we find the matching opening parenthesis
                while !operator_stack.is_empty() {
                    let last = operator_stack
                        .last()
                        .ok_or("Missing operator on stack")?;
                    if last.paren_opener {
                        break;
                    }
                    let op = operator_stack.pop().unwrap();
                    output.push(op.token);
                }

                let last_op = operator_stack
                    .last()
                    .ok_or("Mismatched parentheses")?;

                // Remove the opening parenthesis
                if last_op.token.token_type == TokenType::OpenParen {
                    operator_stack.pop();
                    continue;
                }

                // If there's a function waiting, add it to output
                if !operator_stack.is_empty() {
                    let last = operator_stack
                        .last()
                        .ok_or("Missing operator on stack")?;
                    if last.is_func {
                        let func = operator_stack.pop().unwrap();
                        output.push(func.token);
                    }
                }
            }

            // Operators: apply precedence and associativity rules
            TokenType::Star
            | TokenType::Slash
            | TokenType::Plus
            | TokenType::Minus
            | TokenType::Power
            | TokenType::Modulo
            | TokenType::Percent
            | TokenType::Factorial => {
                let o_1 = get_operator(token);

                // Pop higher precedence operators from stack
                while !operator_stack.is_empty() {
                    let last = operator_stack
                        .last()
                        .ok_or("Missing operator on stack")?;

                    if last.paren_opener {
                        break;
                    }

                    // Precedence and associativity rules
                    let should_pop = last.prec > o_1.prec
                        || (last.prec == o_1.prec && o_1.assoc == Assoc::Left);

                    if !should_pop {
                        break;
                    }

                    let o_2_new = operator_stack.pop().unwrap();
                    output.push(o_2_new.token);
                }

                operator_stack.push(o_1);
            }

            // Operands go directly to output
            TokenType::Number | TokenType::X => output.push(token),

            // End token marks completion
            TokenType::End => {
                found_end = true;
            }

            // Synthetic tokens should never appear in input
            _ => unreachable!("Synthetic token should not be here"),
        }
    }

    // Pop remaining operators from stack
    while let Some(op) = operator_stack.pop() {
        if op.token.token_type == TokenType::OpenParen {
            return Err("Invalid opening parenthesis".to_string());
        }
        output.push(op.token);
    }

    // Validation
    if paren_depth != 0 {
        return Err("Invalid function".to_string());
    }

    if !found_end {
        return Err("No end token found".to_string());
    }

    Ok(output)
}
