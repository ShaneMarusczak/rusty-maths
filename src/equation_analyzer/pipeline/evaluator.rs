use crate::{
    equation_analyzer::structs::token::{Token, TokenType},
    utilities::{abs_f32, factorial, square_root_f32},
};
use std::{
    collections::HashMap,
    f32::consts::{E, PI},
};

/// Evaluates a parsed equation in Reverse Polish Notation (RPN).
///
/// # Arguments
/// * `parsed_eq` - A slice of tokens in RPN format
/// * `x` - Optional value of the variable x (defaults to 0.0 if None)
///
/// # Returns
/// * `Ok(f32)` - The result of the evaluation
/// * `Err(String)` - An error message if evaluation fails
pub(crate) fn evaluate(parsed_eq: &[Token], x: impl Into<Option<f32>>) -> Result<f32, String> {
    let x = x.into().unwrap_or(0.0);
    if parsed_eq.is_empty() {
        return Err(String::from("Invalid equation supplied"));
    }
    let mut stack: Vec<f32> = Vec::with_capacity(parsed_eq.len());

    let mut params = vec![];

    let mut collecting_params = false;

    for token in parsed_eq {
        if collecting_params {
            match token.token_type {
                TokenType::Number => params.push(token.numeric_value_1),
                TokenType::X => params.push(token.numeric_value_1 * x.powf(token.numeric_value_2)),
                TokenType::EndAvg => {
                    let avg = params.iter().sum::<f32>() / params.len() as f32;
                    stack.push(avg);
                }
                TokenType::EndMin => {
                    let min = params.iter().copied().fold(f32::MAX, f32::min);
                    stack.push(min);
                }
                TokenType::EndMax => {
                    let max = params.iter().copied().fold(f32::MIN, f32::max);
                    stack.push(max);
                }
                TokenType::EndChoice => {
                    if params.len() != 2 {
                        return Err(format!(
                            "Choice function takes two parameters, found {}.",
                            params.len()
                        ));
                    }
                    if params.iter().any(|p| p % 1.0 != 0.0) {
                        return Err("Choice is only defined for positive whole numbers".to_string());
                    }
                    let n = params[0] as isize;
                    let k = params[1] as isize;
                    let val = (factorial(n) / (factorial(k) * factorial(n - k))) as f32;
                    stack.push(val);
                }
                TokenType::EndMode => {
                    // Build frequency map
                    let mut seen: HashMap<u32, usize> = HashMap::new();
                    for param in params.iter().clone() {
                        let bits = param.to_bits();
                        let count = seen.entry(bits).or_insert(0);
                        *count += 1;
                    }

                    if seen.is_empty() {
                        return Err("Mode requires at least one parameter".to_string());
                    }

                    let max_count = *seen.values().max().unwrap();

                    // Uniform distribution: all values appear with same frequency
                    // e.g., [1, 2, 3, 4] - each appears once, no mode exists
                    if max_count == 1 {
                        stack.push(f32::NAN);
                    } else {
                        // Collect all values with max frequency (handles multimodal)
                        // e.g., [1, 1, 2, 2, 3] - both 1 and 2 are modes
                        let modes: Vec<f32> = seen.iter()
                            .filter(|(_, &count)| count == max_count)
                            .map(|(&bits, _)| f32::from_bits(bits))
                            .collect();

                        // Return average of all modes
                        let mode_avg = modes.iter().sum::<f32>() / modes.len() as f32;
                        stack.push(mode_avg);
                    }
                }
                TokenType::EndMed => {
                    params.sort_by(|a, b| {
                        a.partial_cmp(b)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });
                    let len = params.len();
                    let med = if len % 2 == 0 {
                        let mid = len / 2;
                        (params[mid - 1] + params[mid]) / 2.0
                    } else {
                        params[len / 2]
                    };
                    stack.push(med);
                }
                _ => unreachable!(),
            }
            if !matches!(token.token_type, TokenType::Number | TokenType::X) {
                collecting_params = false;
                params.clear();
            }
            continue;
        }

        match token.token_type {
            TokenType::Avg
            | TokenType::Max
            | TokenType::Min
            | TokenType::Med
            | TokenType::Mode
            | TokenType::Choice => {
                collecting_params = true;
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
                let temp = stack.pop().ok_or("Insufficient operands for asin function")?;
                stack.push(temp.asin());
            }
            TokenType::Acos => {
                let temp = stack.pop().ok_or("Insufficient operands for acos function")?;
                stack.push(temp.acos());
            }
            TokenType::Atan => {
                let temp = stack.pop().ok_or("Insufficient operands for atan function")?;
                stack.push(temp.atan());
            }
            TokenType::Abs => {
                let temp = stack.pop().ok_or("Insufficient operands for abs function")?;
                stack.push(abs_f32(temp));
            }
            TokenType::Sqrt => {
                let temp = stack.pop().ok_or("Insufficient operands for sqrt function")?;
                if temp.is_sign_negative() {
                    //TODO: For now return NaN, I want to return a complex number at some point
                    return Ok(f32::NAN);
                }
                stack.push(square_root_f32(temp));
            }
            TokenType::Ln => {
                let temp = stack.pop().ok_or("Insufficient operands for ln function")?;
                stack.push(temp.ln());
            }
            TokenType::Factorial => {
                let temp = stack.pop().ok_or("Insufficient operands for factorial operator")?;
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
