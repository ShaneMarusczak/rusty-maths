use crate::equation_analyzer::structs::operands::{get_operator, Assoc, Operand};
use crate::equation_analyzer::structs::token::{ParamToken, Token, TokenType};

/// Parses a vector of tokens into Reverse Polish Notation (RPN) using the Shunting Yard algorithm.
///
/// # Arguments
/// * `tokens` - A vector of tokens in infix notation
///
/// # Returns
/// * `Ok(Vec<Token>)` - The tokens in RPN format
/// * `Err(String)` - An error message if parsing fails
///
/// # Algorithm
/// Uses Dijkstra's Shunting Yard algorithm to convert infix notation to RPN,
/// which allows for efficient evaluation without ambiguity.
pub(crate) fn parse(tokens: Vec<Token>) -> Result<Vec<Token>, String> {
    let mut operator_stack: Vec<Operand> = Vec::with_capacity(tokens.len());

    let mut output: Vec<Token> = Vec::with_capacity(tokens.len());

    let mut paren_depth = 0;

    let mut param_token = ParamToken::None;

    let mut found_end = false;

    for token in tokens {
        if param_token != ParamToken::None {
            if matches!(token.token_type, TokenType::Number) {
                output.push(token);
                continue;
            } else if token.token_type == TokenType::Comma {
                continue;
            } else if token.token_type == TokenType::CloseParen {
                match param_token {
                    ParamToken::Avg => {
                        output.push(Token {
                            token_type: TokenType::EndAvg,
                            numeric_value_1: 0_f32,
                            numeric_value_2: 0_f32,
                        });
                    }
                    ParamToken::Choice => {
                        output.push(Token {
                            token_type: TokenType::EndChoice,
                            numeric_value_1: 0_f32,
                            numeric_value_2: 0_f32,
                        });
                    }
                    ParamToken::Med => {
                        output.push(Token {
                            token_type: TokenType::EndMed,
                            numeric_value_1: 0_f32,
                            numeric_value_2: 0_f32,
                        });
                    }
                    ParamToken::Min => {
                        output.push(Token {
                            token_type: TokenType::EndMin,
                            numeric_value_1: 0_f32,
                            numeric_value_2: 0_f32,
                        });
                    }
                    ParamToken::Max => {
                        output.push(Token {
                            token_type: TokenType::EndMax,
                            numeric_value_1: 0_f32,
                            numeric_value_2: 0_f32,
                        });
                    }
                    ParamToken::Mode => {
                        output.push(Token {
                            token_type: TokenType::EndMode,
                            numeric_value_1: 0_f32,
                            numeric_value_2: 0_f32,
                        });
                    }
                    ParamToken::None => unreachable!(),
                }

                param_token = ParamToken::None;
                continue;
            }

            return Err("Params can only be numbers".to_string());
        }

        match token.token_type {
            TokenType::Y | TokenType::Equal | TokenType::Comma => continue,

            TokenType::_Pi | TokenType::_E | TokenType::NegPi | TokenType::NegE => {
                output.push(token);
            }

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

            TokenType::CloseParen => {
                paren_depth -= 1;

                if paren_depth < 0 {
                    return Err("Invalid closing parenthesis".to_string());
                }

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

                if last_op.token.token_type == TokenType::OpenParen {
                    operator_stack.pop();
                    continue;
                }

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

            TokenType::Star
            | TokenType::Slash
            | TokenType::Plus
            | TokenType::Minus
            | TokenType::Power
            | TokenType::Modulo
            | TokenType::Percent
            | TokenType::Factorial => {
                let o_1 = get_operator(token);
                while !operator_stack.is_empty() {
                    let last = operator_stack
                        .last()
                        .ok_or("Missing operator on stack")?;

                    if last.paren_opener {
                        break;
                    }

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

            TokenType::Number => output.push(token),
            TokenType::End => {
                found_end = true;
            }
            _ => unreachable!("Synthetic token should not be here"),
        }
    }

    while let Some(op) = operator_stack.pop() {
        if op.token.token_type == TokenType::OpenParen {
            return Err("Invalid opening parenthesis".to_string());
        }
        output.push(op.token);
    }

    if paren_depth != 0 {
        return Err("Invalid function".to_string());
    }

    if !found_end {
        return Err("No end token found".to_string());
    }

    Ok(output)
}
