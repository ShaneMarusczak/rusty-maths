use crate::{
    equation_analyzer::structs::token::{Token, TokenType},
    utilities::{abs_f32, square_root_f32},
};
use std::{
    collections::HashMap,
    f32::consts::{E, PI},
};

pub(crate) fn evaluate(parsed_eq: &[Token], x: f32) -> Result<f32, String> {
    let mut stack: Vec<f32> = Vec::with_capacity(parsed_eq.len());

    let mut params = vec![];

    let mut collecting_params = false;

    for token in parsed_eq {
        if collecting_params {
            if token.token_type == TokenType::Number {
                params.push(token.numeric_value_1);
            } else if token.token_type == TokenType::EndAvg {
                collecting_params = false;
                let avg = params.iter().sum::<f32>() / params.len() as f32;
                stack.push(avg);
                params.clear();
            } else if token.token_type == TokenType::EndMin {
                collecting_params = false;
                let min = params.iter().cloned().fold(f32::MAX, f32::min);
                stack.push(min);
                params.clear();
            } else if token.token_type == TokenType::EndMax {
                collecting_params = false;
                let max = params.iter().cloned().fold(f32::MIN, f32::max);
                stack.push(max);
                params.clear();
            } else if token.token_type == TokenType::EndMode {
                //TODO: uniform and multimodal
                //uniform - all values appear the same amount [1,2,3,4,5,6]
                //multimodal - tie with 2 or more values that appear more than the rest [1,1,2,2,3,4,5] {1,2}
                collecting_params = false;
                let mut seen: HashMap<u32, usize> = HashMap::new();
                for param in params.iter().clone() {
                    let bits = param.to_bits();
                    let count = seen.entry(bits).or_insert(0);
                    *count += 1;
                }

                let mut max_count = 0;
                let mut max = 0;
                for (key, value) in seen {
                    if value > max_count {
                        max_count = value;
                        max = key;
                    };
                }
                let mode = f32::from_bits(max);
                stack.push(mode);
                params.clear();
            } else if token.token_type == TokenType::EndMed {
                collecting_params = false;

                params.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let len = params.len();

                let med = if len % 2 == 0 {
                    let mid = len / 2;
                    (params[mid - 1] + params[mid]) / 2.0
                } else {
                    params[len / 2]
                };
                stack.push(med);
                params.clear();
            }
            continue;
        }

        match token.token_type {
            TokenType::Avg | TokenType::Max | TokenType::Min | TokenType::Med | TokenType::Mode => {
                collecting_params = true;
            }
            TokenType::Number => stack.push(token.numeric_value_1),
            TokenType::_Pi => stack.push(PI),
            TokenType::_E => stack.push(E),
            TokenType::NegPi => stack.push(-PI),
            TokenType::NegE => stack.push(-E),
            TokenType::Sin => {
                let temp = stack.pop().unwrap();
                stack.push(temp.sin());
            }
            TokenType::Cos => {
                let temp = stack.pop().unwrap();
                stack.push(temp.cos());
            }
            TokenType::Tan => {
                let temp = stack.pop().unwrap();
                stack.push(temp.tan());
            }
            TokenType::Abs => {
                let temp = stack.pop().unwrap();
                stack.push(abs_f32(temp));
            }
            TokenType::Sqrt => {
                let temp = stack.pop().unwrap();
                stack.push(square_root_f32(temp));
            }
            TokenType::Ln => {
                let temp = stack.pop().unwrap();
                stack.push(temp.ln());
            }
            TokenType::Factorial => {
                let temp = stack.pop().unwrap();
                if temp % 1.0 != 0.0 {
                    return Err("Factorial is only defined for positive whole numbers".to_string());
                }
                stack.push(crate::utilities::factorial(temp as isize) as f32);
            }
            TokenType::Log => {
                let temp = stack.pop().unwrap();
                stack.push(temp.log(token.numeric_value_1));
            }
            TokenType::X => stack.push(token.numeric_value_1 * x.powf(token.numeric_value_2)),
            _ => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
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
            }
        }
    }
    if stack.len() != 1 {
        return Err("Invalid evaluation stack, big boo boo".to_string());
    }
    Ok(stack[0])
}
