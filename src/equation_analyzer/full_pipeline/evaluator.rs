use crate::{
    equation_analyzer::structs::token::{Token, TokenType},
    equation_analyzer::utils::param_collector::{CollectionResult, ParamCollector},
    utilities::{abs_f32, square_root_f32},
};
use std::f32::consts::{E, PI};

/// Evaluates RPN tokens from an iterator, maintaining only a minimal value stack.
///
/// This evaluator pulls tokens one at a time from the parser iterator and evaluates
/// them incrementally. It only buffers the value stack (partial buffer), not all tokens.
///
/// # Arguments
/// * `tokens` - An iterator yielding RPN tokens
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
    I: Iterator<Item = Result<Token, String>>,
{
    let x = x.into().unwrap_or(0.0);
    let mut stack: Vec<f32> = Vec::new();
    let mut collector = ParamCollector::new();

    for token_result in tokens {
        let token = token_result?;

        // Check if ParamCollector wants to handle this token
        match collector.process_token(&token, x) {
            CollectionResult::NotCollecting => {
                // Fall through to normal token processing
            }
            CollectionResult::Continue => {
                continue;
            }
            CollectionResult::Finished(Ok(value)) => {
                stack.push(value);
                continue;
            }
            CollectionResult::Finished(Err(e)) => {
                return Err(e);
            }
        }

        match token.token_type {
            _ if token.token_type.is_variadic_function() => {
                collector.start_collecting();
            }
            TokenType::Number => stack.push(token.numeric_value_1),
            TokenType::_Pi => stack.push(PI),
            TokenType::_E => stack.push(E),
            TokenType::NegPi => stack.push(-PI),
            TokenType::NegE => stack.push(-E),
            TokenType::Sin => {
                let temp = stack.pop().ok_or("Insufficient operands for sin function")?;
                stack.push(temp.sin());
            }
            TokenType::Cos => {
                let temp = stack.pop().ok_or("Insufficient operands for cos function")?;
                stack.push(temp.cos());
            }
            TokenType::Tan => {
                let temp = stack.pop().ok_or("Insufficient operands for tan function")?;
                stack.push(temp.tan());
            }
            TokenType::Asin => {
                let temp = stack
                    .pop()
                    .ok_or("Insufficient operands for asin function")?;
                stack.push(temp.asin());
            }
            TokenType::Acos => {
                let temp = stack
                    .pop()
                    .ok_or("Insufficient operands for acos function")?;
                stack.push(temp.acos());
            }
            TokenType::Atan => {
                let temp = stack
                    .pop()
                    .ok_or("Insufficient operands for atan function")?;
                stack.push(temp.atan());
            }
            TokenType::Abs => {
                let temp = stack.pop().ok_or("Insufficient operands for abs function")?;
                stack.push(abs_f32(temp));
            }
            TokenType::Sqrt => {
                let temp = stack
                    .pop()
                    .ok_or("Insufficient operands for sqrt function")?;
                if temp.is_sign_negative() {
                    return Ok(f32::NAN);
                }
                stack.push(square_root_f32(temp));
            }
            TokenType::Ln => {
                let temp = stack.pop().ok_or("Insufficient operands for ln function")?;
                stack.push(temp.ln());
            }
            TokenType::Factorial => {
                let temp = stack
                    .pop()
                    .ok_or("Insufficient operands for factorial operator")?;
                if temp % 1.0 != 0.0 {
                    return Err("Factorial is only defined for positive whole numbers".to_string());
                }
                stack.push(crate::utilities::factorial(temp as isize) as f32);
            }
            TokenType::Log => {
                let temp = stack.pop().ok_or("Insufficient operands for log function")?;
                stack.push(temp.log(token.numeric_value_1));
            }
            TokenType::X => stack.push(token.numeric_value_1 * x.powf(token.numeric_value_2)),
            _ => {
                if let (Some(rhs), Some(lhs)) = (stack.pop(), stack.pop()) {
                    match token.token_type {
                        TokenType::Plus => stack.push(lhs + rhs),
                        TokenType::Minus => stack.push(lhs - rhs),
                        TokenType::Star => stack.push(lhs * rhs),
                        TokenType::Slash => stack.push(lhs / rhs),
                        TokenType::Modulo => stack.push(lhs % rhs),
                        TokenType::Percent => {
                            let hundredth_of_rhs = rhs / 100_f32;
                            stack.push(lhs * hundredth_of_rhs);
                        }
                        TokenType::Power => stack.push(lhs.powf(rhs)),
                        _ => return Err(format!("Unknown token: {:?}", token)),
                    }
                } else {
                    return Err("Invalid expression".to_string());
                }
            }
        }
    }

    if stack.is_empty() {
        return Err("Evaluation stack is empty".to_string());
    }

    if stack.len() != 1 {
        return Err(format!(
            "Invalid evaluation: expected 1 result, found {} items in stack",
            stack.len()
        ));
    }

    stack
        .pop()
        .ok_or_else(|| "Evaluation stack is empty".to_string())
}
