use crate::utilities::{abs_f32, square_root_f32};
use std::f32::consts::{E, PI};

pub fn evaluate(parsed_eq: &[String], x: f32) -> Result<f32, String> {
    let mut stack: Vec<f32> = Vec::with_capacity(parsed_eq.len());
    for token in parsed_eq {
        if let Ok(n) = token.parse() {
            stack.push(n);
        } else {
            match token.as_str() {
                "π" => stack.push(PI),
                "e" => stack.push(E),
                "-π" => stack.push(-PI),
                "-e" => stack.push(-E),
                "sin(" => {
                    let temp = stack.pop().unwrap();
                    stack.push(temp.sin());
                }
                "cos(" => {
                    let temp = stack.pop().unwrap();
                    stack.push(temp.cos());
                }
                "tan(" => {
                    let temp = stack.pop().unwrap();
                    stack.push(temp.tan());
                }
                "abs(" => {
                    let temp = stack.pop().unwrap();
                    stack.push(abs_f32(temp));
                }
                "sqrt(" => {
                    let temp = stack.pop().unwrap();
                    stack.push(square_root_f32(temp));
                }
                "ln(" => {
                    let temp = stack.pop().unwrap();
                    stack.push(temp.ln());
                }
                _ => {
                    if token.starts_with("log_") {
                        let mut new_token = token.to_string();
                        new_token.pop();
                        let base = new_token.split('_').nth(1).unwrap().parse::<f32>().unwrap();
                        let temp = stack.pop().unwrap();
                        stack.push(temp.log(base));
                    } else if token.contains("x^") {
                        let split_token = token.split('x').collect::<Vec<&str>>();

                        let coefficient = split_token[0].parse::<f32>().unwrap();
                        let pow = split_token[1][1..].parse::<f32>().unwrap();

                        stack.push(coefficient * x.powf(pow));
                    } else {
                        let rhs = stack.pop().unwrap();
                        let lhs = stack.pop().unwrap();
                        match token.as_str() {
                            "+" => stack.push(lhs + rhs),
                            "-" => stack.push(lhs - rhs),
                            "*" => stack.push(lhs * rhs),
                            "/" => stack.push(lhs / rhs),
                            "%" => stack.push(lhs % rhs),
                            "%%" => {
                                let hundredth_of_rhs = rhs / 100_f32;
                                stack.push(lhs * hundredth_of_rhs);
                            },
                            "^" => stack.push(lhs.powf(rhs)),
                            "max(" => stack.push(lhs.max(rhs)),
                            "min(" => stack.push(lhs.min(rhs)),
                            _ => return Err(format!("Unknown token: {}", token)),
                        }
                    }
                }
            }
        }
    }
    if stack.len() != 1 {
        return Err(format!("Invalid evaluation stack, big boo boo"));
    }
    Ok(stack[0])
}
