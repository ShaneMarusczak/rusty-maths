use crate::equation_analyzer::structs::operands::{get_operator, Assoc, Operand};
use crate::equation_analyzer::structs::token::{Token, TokenType};
use crate::equation_analyzer::utils::make_synthetic_token;

/// Represents a parser frame for variadic functions.
/// Tracks the function type and where its operators begin on the stack.
struct ParserFrame {
    token_type: TokenType,
    operator_stack_position: usize,
}

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
    let mut frames: Vec<ParserFrame> = Vec::new();
    let mut found_end = false;
    let mut expect_piped_function = false;

    for token_result in tokens {
        let token = token_result?;

        // After a Pipe, the next token must be a unary function token.
        // Emit it directly to the output queue (it's already in postfix position).
        if expect_piped_function {
            match token.token_type {
                TokenType::Sin
                | TokenType::Cos
                | TokenType::Tan
                | TokenType::Asin
                | TokenType::Acos
                | TokenType::Atan
                | TokenType::Sinh
                | TokenType::Cosh
                | TokenType::Tanh
                | TokenType::Sec
                | TokenType::Csc
                | TokenType::Cot
                | TokenType::Deg
                | TokenType::Rad
                | TokenType::Abs
                | TokenType::Sqrt
                | TokenType::Ln => {
                    output.push(token);
                    expect_piped_function = false;
                    continue;
                }
                _ => {
                    return Err(format!(
                        "Expected a unary function after '|>', got {:?}",
                        token.token_type
                    ));
                }
            }
        }

        // Handle variadic function parameter collection
        // With frame-based evaluation, we now allow full expressions in parameters
        if let Some(frame) = frames.last() {
            match token.token_type {
                // Comma: pop all pending operators (they belong to current parameter expression)
                TokenType::Comma => {
                    while !operator_stack.is_empty() {
                        let last = operator_stack.last().ok_or("Missing operator on stack")?;
                        if last.paren_opener {
                            break;
                        }
                        let op = operator_stack.pop().ok_or("Operator stack empty")?;
                        output.push(op.token);
                    }
                    continue;
                }

                // CloseParen might end the variadic function, or a nested regular function
                TokenType::CloseParen => {
                    // Pop all operators until we find a paren_opener
                    while !operator_stack.is_empty() {
                        let last = operator_stack.last().ok_or("Missing operator on stack")?;
                        if last.paren_opener {
                            break;
                        }
                        let op = operator_stack.pop().ok_or("Operator stack empty")?;
                        output.push(op.token);
                    }

                    // Check if we've drained back to the frame boundary
                    if operator_stack.len() == frame.operator_stack_position + 1 {
                        operator_stack.pop();
                        paren_depth -= 1;
                        output.push(make_synthetic_token(frame.token_type.to_end_token_type()));
                        frames.pop();
                        continue;
                    }

                    // Otherwise, it's a regular function or parenthesis - fall through to normal processing
                }

                // All other tokens fall through to normal processing
                _ => {}
            }
        }

        match token.token_type {
            // Skip equation markers and commas
            TokenType::Y | TokenType::Equal | TokenType::Comma => continue,

            // Constants go directly to output
            TokenType::_Pi | TokenType::_E => {
                output.push(token);
            }

            // Variadic functions - start parameter collection
            // The tokenizer consumes the OpenParen, so we push a synthetic marker
            // to the operator stack to fence off preceding operators
            _ if token.token_type.is_variadic_function() => {
                output.push(token);
                frames.push(ParserFrame {
                    token_type: token.token_type,
                    operator_stack_position: operator_stack.len(),
                });
                operator_stack.push(get_operator(make_synthetic_token(TokenType::OpenParen))?);
                paren_depth += 1;
            }

            // Functions and opening parenthesis go on operator stack
            TokenType::Sin
            | TokenType::Cos
            | TokenType::Tan
            | TokenType::Asin
            | TokenType::Acos
            | TokenType::Atan
            | TokenType::Sinh
            | TokenType::Cosh
            | TokenType::Tanh
            | TokenType::Sec
            | TokenType::Csc
            | TokenType::Cot
            | TokenType::Deg
            | TokenType::Rad
            | TokenType::Abs
            | TokenType::Sqrt
            | TokenType::Ln
            | TokenType::Log
            | TokenType::OpenParen => {
                paren_depth += 1;
                operator_stack.push(get_operator(token)?);
            }

            // Closing parenthesis: pop operators until matching open paren
            TokenType::CloseParen => {
                paren_depth -= 1;

                if paren_depth < 0 {
                    return Err("Invalid closing parenthesis".to_string());
                }

                // Pop operators until we find the matching opening parenthesis
                while !operator_stack.is_empty() {
                    let last = operator_stack.last().ok_or("Missing operator on stack")?;
                    if last.paren_opener {
                        break;
                    }
                    let op = operator_stack.pop().ok_or_else(|| {
                        String::from("Internal error: operator stack became empty")
                    })?;
                    output.push(op.token);
                }

                let last_op = operator_stack.last().ok_or("Mismatched parentheses")?;

                // Remove the opening parenthesis
                if last_op.token.token_type == TokenType::OpenParen {
                    operator_stack.pop();
                    continue;
                }

                // If there's a function waiting, add it to output
                if !operator_stack.is_empty() {
                    let last = operator_stack.last().ok_or("Missing operator on stack")?;
                    if last.is_func {
                        let func = operator_stack.pop().ok_or_else(|| {
                            String::from("Internal error: operator stack became empty")
                        })?;
                        output.push(func.token);
                    }
                }
            }

            // Operators: apply precedence and associativity rules
            TokenType::Star
            | TokenType::Slash
            | TokenType::Plus
            | TokenType::Minus
            | TokenType::UnaryMinus
            | TokenType::Power
            | TokenType::Modulo
            | TokenType::Percent
            | TokenType::Factorial => {
                let o_1 = get_operator(token)?;

                // Pop higher precedence operators from stack
                while !operator_stack.is_empty() {
                    let last = operator_stack.last().ok_or("Missing operator on stack")?;

                    if last.paren_opener {
                        break;
                    }

                    // Precedence and associativity rules
                    let should_pop =
                        last.prec > o_1.prec || (last.prec == o_1.prec && o_1.assoc == Assoc::Left);

                    if !should_pop {
                        break;
                    }

                    let o_2_new = operator_stack.pop().ok_or_else(|| {
                        String::from("Internal error: operator stack became empty")
                    })?;
                    output.push(o_2_new.token);
                }

                operator_stack.push(o_1);
            }

            // Pipe operator: flush pending operators (down to current paren depth)
            // and arm `expect_piped_function` so the next token is emitted directly.
            TokenType::Pipe => {
                while let Some(last) = operator_stack.last() {
                    if last.paren_opener {
                        break;
                    }
                    let op = operator_stack.pop().ok_or_else(|| {
                        String::from("Internal error: operator stack became empty")
                    })?;
                    output.push(op.token);
                }
                expect_piped_function = true;
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

    if expect_piped_function {
        return Err("Dangling '|>': expected a unary function on the right side".to_string());
    }

    Ok(output)
}
