use crate::{
    equation_analyzer::structs::token::{Token, TokenType},
    utilities::{abs_f32, factorial, square_root_f32},
};
use std::collections::HashMap;
use std::f32::consts::{E, PI};

/// Represents a function call frame for variadic functions
struct FunctionFrame {
    /// Position in the stack where this function's parameters start
    stack_position: usize,
}

/// Pops the current function frame and splits off its parameters from the stack.
fn pop_frame(
    frames: &mut Vec<FunctionFrame>,
    stack: &mut Vec<f32>,
    name: &str,
) -> Result<Vec<f32>, String> {
    let frame = frames
        .pop()
        .ok_or_else(|| format!("Unexpected End{} token", name))?;
    Ok(stack.split_off(frame.stack_position))
}

/// Generic RPN evaluator that works with any iterator of tokens.
///
/// This is the core evaluation logic shared by all pipeline implementations.
/// It evaluates mathematical expressions in Reverse Polish Notation using a stack-based algorithm.
///
/// # Arguments
/// * `tokens` - An iterator of tokens in RPN format
/// * `x` - Optional value of the variable x (defaults to 0.0 if None)
///
/// # Returns
/// * `Ok(f32)` - The result of the evaluation
/// * `Err(String)` - An error message if evaluation fails
///
/// # Algorithm
/// 1. Maintains a value stack for intermediate results
/// 2. Uses frame markers to handle variadic functions (avg, min, max, etc.)
/// 3. Processes each token:
///    - Numbers/Constants: Push to stack
///    - Unary operators: Pop, apply, push
///    - Binary operators: Pop twice, apply, push
///    - Variadic functions: Mark stack position with frame
///    - End* tokens: Collect params since frame position, compute, push result
/// 4. Returns final stack value (should be exactly 1 value)
pub(crate) fn evaluate<I>(tokens: I, x: impl Into<Option<f32>>) -> Result<f32, String>
where
    I: IntoIterator<Item = Token>,
{
    let x = x.into().unwrap_or(0.0);
    let mut stack: Vec<f32> = Vec::new();
    let mut frames: Vec<FunctionFrame> = Vec::new();
    let mut token_count = 0;

    for token in tokens {
        token_count += 1;

        match token.token_type {
            // Variadic functions - push a frame marker
            _ if token.token_type.is_variadic_function() => {
                frames.push(FunctionFrame {
                    stack_position: stack.len(),
                });
            }
            // End tokens - collect params and compute
            TokenType::EndMin => {
                let params = pop_frame(&mut frames, &mut stack, "Min")?;
                if params.is_empty() {
                    return Err("min requires at least one parameter".to_string());
                }
                stack.push(params.iter().copied().fold(f32::MAX, f32::min));
            }
            TokenType::EndMax => {
                let params = pop_frame(&mut frames, &mut stack, "Max")?;
                if params.is_empty() {
                    return Err("max requires at least one parameter".to_string());
                }
                stack.push(params.iter().copied().fold(f32::MIN, f32::max));
            }
            TokenType::EndAvg => {
                let params = pop_frame(&mut frames, &mut stack, "Avg")?;
                if params.is_empty() {
                    return Err("avg requires at least one parameter".to_string());
                }
                stack.push(params.iter().sum::<f32>() / params.len() as f32);
            }
            TokenType::EndMed => {
                let mut params = pop_frame(&mut frames, &mut stack, "Med")?;
                if params.is_empty() {
                    return Err("median requires at least one parameter".to_string());
                }
                params.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                let len = params.len();
                let result = if len.is_multiple_of(2) {
                    let mid = len / 2;
                    (params[mid - 1] + params[mid]) / 2.0
                } else {
                    params[len / 2]
                };
                stack.push(result);
            }
            TokenType::EndMode => {
                let params = pop_frame(&mut frames, &mut stack, "Mode")?;
                if params.is_empty() {
                    return Err("mode requires at least one parameter".to_string());
                }

                // Build frequency map
                let mut seen: HashMap<u32, usize> = HashMap::new();
                for param in params.iter() {
                    let bits = param.to_bits();
                    *seen.entry(bits).or_insert(0) += 1;
                }

                let max_count = *seen
                    .values()
                    .max()
                    .ok_or_else(|| String::from("mode requires at least one parameter"))?;

                let result = if max_count == 1 {
                    // Uniform distribution
                    f32::NAN
                } else {
                    // Collect all modes and return average
                    let modes: Vec<f32> = seen
                        .iter()
                        .filter(|(_, &count)| count == max_count)
                        .map(|(&bits, _)| f32::from_bits(bits))
                        .collect();
                    modes.iter().sum::<f32>() / modes.len() as f32
                };
                stack.push(result);
            }
            TokenType::EndAtan2 => {
                let params = pop_frame(&mut frames, &mut stack, "Atan2")?;
                if params.len() != 2 {
                    return Err(format!(
                        "atan2 requires exactly 2 parameters (y, x), got {}",
                        params.len()
                    ));
                }
                stack.push(params[0].atan2(params[1]));
            }
            TokenType::EndChoice => {
                let params = pop_frame(&mut frames, &mut stack, "Choice")?;
                if params.len() != 2 {
                    return Err(format!(
                        "choice requires exactly 2 parameters, got {}",
                        params.len()
                    ));
                }

                // Validate integers
                for (i, &param) in params.iter().enumerate() {
                    if param % 1.0 != 0.0 {
                        return Err(format!(
                            "Parameter {} must be an integer, got {}",
                            i + 1,
                            param
                        ));
                    }
                    if param < 0.0 {
                        return Err(format!(
                            "Parameter {} must be non-negative, got {}",
                            i + 1,
                            param
                        ));
                    }
                }

                let n = params[0] as isize;
                let k = params[1] as isize;

                let result = if k > n {
                    0.0
                } else {
                    (factorial(n) / (factorial(k) * factorial(n - k))) as f32
                };
                stack.push(result);
            }
            TokenType::Number => stack.push(token.numeric_value_1),
            TokenType::_Pi => stack.push(PI),
            TokenType::_E => stack.push(E),
            TokenType::UnaryMinus => {
                let temp = stack
                    .pop()
                    .ok_or("Insufficient operands for unary minus operator")?;
                stack.push(-temp);
            }
            TokenType::Sin => {
                let temp = stack
                    .pop()
                    .ok_or("Insufficient operands for sin function")?;
                stack.push(temp.sin());
            }
            TokenType::Cos => {
                let temp = stack
                    .pop()
                    .ok_or("Insufficient operands for cos function")?;
                stack.push(temp.cos());
            }
            TokenType::Tan => {
                let temp = stack
                    .pop()
                    .ok_or("Insufficient operands for tan function")?;
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
            TokenType::Sinh => {
                let temp = stack
                    .pop()
                    .ok_or("Insufficient operands for sinh function")?;
                stack.push(temp.sinh());
            }
            TokenType::Cosh => {
                let temp = stack
                    .pop()
                    .ok_or("Insufficient operands for cosh function")?;
                stack.push(temp.cosh());
            }
            TokenType::Tanh => {
                let temp = stack
                    .pop()
                    .ok_or("Insufficient operands for tanh function")?;
                stack.push(temp.tanh());
            }
            TokenType::Sec => {
                let temp = stack
                    .pop()
                    .ok_or("Insufficient operands for sec function")?;
                stack.push(1.0 / temp.cos());
            }
            TokenType::Csc => {
                let temp = stack
                    .pop()
                    .ok_or("Insufficient operands for csc function")?;
                stack.push(1.0 / temp.sin());
            }
            TokenType::Cot => {
                let temp = stack
                    .pop()
                    .ok_or("Insufficient operands for cot function")?;
                stack.push(1.0 / temp.tan());
            }
            TokenType::Deg => {
                let temp = stack
                    .pop()
                    .ok_or("Insufficient operands for deg function")?;
                stack.push(temp * 180.0 / PI);
            }
            TokenType::Rad => {
                let temp = stack
                    .pop()
                    .ok_or("Insufficient operands for rad function")?;
                stack.push(temp * PI / 180.0);
            }
            TokenType::Abs => {
                let temp = stack
                    .pop()
                    .ok_or("Insufficient operands for abs function")?;
                stack.push(abs_f32(temp));
            }
            TokenType::Sqrt => {
                let temp = stack
                    .pop()
                    .ok_or("Insufficient operands for sqrt function")?;
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
                let temp = stack
                    .pop()
                    .ok_or("Insufficient operands for factorial operator")?;
                if temp < 0.0 || temp % 1.0 != 0.0 {
                    return Err("Factorial is only defined for non-negative integers".to_string());
                }
                stack.push(crate::utilities::factorial(temp as isize) as f32);
            }
            TokenType::Log => {
                let temp = stack
                    .pop()
                    .ok_or("Insufficient operands for log function")?;
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

    if token_count == 0 {
        return Err(String::from("Invalid equation supplied"));
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
