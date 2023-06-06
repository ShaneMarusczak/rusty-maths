use crate::{
    equation_analyzer::structs::token::{Token, TokenType},
    utilities::{abs_f32, square_root_f32},
};
use std::f32::consts::{E, PI};

pub(crate) fn evaluate(parsed_eq: &[Token], x: f32) -> Result<f32, String> {
    let mut stack: Vec<f32> = Vec::with_capacity(parsed_eq.len());
    for token in parsed_eq {
        match token.token_type {
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
                    TokenType::Max => stack.push(lhs.max(rhs)),
                    TokenType::Min => stack.push(lhs.min(rhs)),
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
