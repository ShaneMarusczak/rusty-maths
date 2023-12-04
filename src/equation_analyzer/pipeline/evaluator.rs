use crate::{
    equation_analyzer::structs::token::{Token, TokenType},
    utilities::{abs_f32, factorial, square_root_f32},
};
use std::{
    collections::HashMap,
    f32::consts::{E, PI},
};

pub(crate) fn evaluate(parsed_eq: &[Token]) -> Result<f32, String> {
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
                    //TODO: uniform and multimodal
                    //uniform - all values appear the same amount [1,2,3,4,5,6]
                    //multimodal - tie with 2 or more values that appear more than the rest [1,1,2,2,3,4,5] {1,2}
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
                }
                TokenType::EndMed => {
                    params.sort_by(|a, b| a.partial_cmp(b).unwrap());
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
            if !matches!(token.token_type, TokenType::Number) {
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
            TokenType::Asin => {
                let temp = stack.pop().unwrap();
                stack.push(temp.asin());
            }
            TokenType::Acos => {
                let temp = stack.pop().unwrap();
                stack.push(temp.acos());
            }
            TokenType::Atan => {
                let temp = stack.pop().unwrap();
                stack.push(temp.atan());
            }
            TokenType::Abs => {
                let temp = stack.pop().unwrap();
                stack.push(abs_f32(temp));
            }
            TokenType::Sqrt => {
                let temp = stack.pop().unwrap();
                if temp.is_sign_negative() {
                    return Err("Cannot take the sqrt of a negative number".to_string());
                }
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
        return Err("Invalid evaluation stack, big boo boo".to_string());
    }
    Ok(stack[0])
}
