use std::f32::consts::{E, PI};
use crate::equation_analyzer::operands::{get_operator, Operand};
use crate::utilities::{abs_f32, square_root_f32};

pub fn eval_rpn(tokens: &Vec<String>, x: f32) -> Result<f32, String> {
    let mut stack: Vec<f32> = Vec::with_capacity(tokens.len());
    for token in tokens {
        if let Ok(n) = token.parse() {
            stack.push(n);
        }
        else if *token == "π" {
            stack.push(PI);
        }
        else if *token == "e" {
            stack.push(E);
        }
        else if *token == "x" {
            stack.push(x);
        }
        else if *token == "sin" {
            let temp = stack.pop().unwrap();
            stack.push(temp.sin());
        }
        else if *token == "cos" {
            let temp = stack.pop().unwrap();
            stack.push(temp.cos());
        }
        else if *token == "tan" {
            let temp = stack.pop().unwrap();
            stack.push(temp.tan());
        }
        else if *token == "abs" {
            let temp = stack.pop().unwrap();
            stack.push(abs_f32(temp));
        }
        else if *token == "sqrt" {
            let temp = stack.pop().unwrap();
            stack.push(square_root_f32(temp));
        }
        else if *token == "ln" {
            let temp = stack.pop().unwrap();
            stack.push(temp.ln());
        }
        else if token.starts_with("log_") {
            let base = token.split('_').nth(1).unwrap().parse::<f32>().unwrap();
            let temp = stack.pop().unwrap();
            stack.push(temp.log(base));
        }
        else {
            let rhs = stack.pop().unwrap();
            let lhs = stack.pop().unwrap();
            match token.as_str() {
                "+" => stack.push(lhs + rhs),
                "-" => stack.push(lhs - rhs),
                "*" => stack.push(lhs * rhs),
                "/" => stack.push(lhs / rhs),
                "^" => stack.push(lhs.powf(rhs)),
                "max" => stack.push(lhs.max(rhs)),
                "min" => stack.push(lhs.min(rhs)),
                _ => return Err(format!("Unknown token: {}", token))

            }
        }
    }
    Ok(stack[0])
}

pub fn get_rpn(eq: &str) -> Result<Vec<String>, String> {
    let mut operator_stack: Vec<Operand> = Vec::with_capacity(eq.len());

    let mut output : Vec<String> = Vec::with_capacity(eq.len());

    let mut paren_depth = 0;

    let mut skip_next = false;

    for (i , term) in eq.to_lowercase().split_whitespace().enumerate() {
        if skip_next {
            skip_next = false;
            continue;
        }
        match term {
            "y" | "=" | "," => continue,
            "π" | "e" => output.push(term.to_string()),
            "sin" | "cos" | "tan" | "(" | "max" | "abs" | "sqrt" | "min" | "ln" => {
                if term == "(" {
                    paren_depth += 1;
                }

                operator_stack.push(get_operator(&term));
            },
            "*" | "/" | "+" | "-" | "^" => {
                //TODO: Write this cleaner
                let o_1 = get_operator(&term);
                while !operator_stack.is_empty() && operator_stack.last().unwrap().token != "(" &&
                    ( operator_stack.last().unwrap().prec > o_1.prec || (operator_stack.last().unwrap().prec == o_1.prec && o_1.assoc == "l")) {
                    let o_2_new = operator_stack.pop().unwrap();
                    output.push(o_2_new.token.to_string());
                }

                operator_stack.push(o_1);
            },
            ")" => {
                paren_depth -= 1;

                if paren_depth < 0 {
                    return Err(format!("Invalid Closing Parenthesis at character {}", i + 1));
                }

                while operator_stack.last().unwrap().token != "(" {
                    let op = operator_stack.pop().unwrap();
                    output.push(op.token.to_string());
                }

                operator_stack.pop();

                if !operator_stack.is_empty() && operator_stack.last().unwrap().is_func {
                    let func = operator_stack.pop().unwrap();
                    output.push(func.token.to_string());
                }
            }
            //TODO: What about 2x? (even if it "should" be written as 2 * x)
            "x" => output.push(term.to_string()),
            _ => {
                if term.parse::<f32>().is_ok() {
                    output.push(term.to_string());
                } else if term.starts_with("log_") {
                    if term.split('_').nth(1).unwrap().parse::<f32>().is_err() {
                        return  Err(String::from("Invalid use of log"))
                    }
                    operator_stack.push(get_operator(&term));
                }
                else{
                    return Err(format!("Unknown term: {}", term));
                }
            }
        }
    }

    while !operator_stack.is_empty() {
        let op = operator_stack.pop().unwrap();
        if op.token == "(" {
            return Err("Invalid Opening Parenthesis".to_string());
        }
        output.push(op.token.to_string());
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_close(x1: f32, x2: f32) -> bool {
        (x1 - x2).abs() < 0.00001
    }

    #[test]
    fn get_rpn_invalid_closing_parens_test(){
        let test = "( 3 + 4 ) )";
        assert_eq!(get_rpn(test).unwrap_err(), String::from("Invalid Closing Parenthesis at character 6"));
    }

    #[test]
    fn get_rpn_invalid_closing_parens_test_2(){
        let test = ")";
        assert_eq!(get_rpn(test).unwrap_err(), String::from("Invalid Closing Parenthesis at character 1"));
    }

    #[test]
    fn get_rpn_empty_parens_test(){
        let test = "( )";
        assert!(get_rpn(test).unwrap().is_empty());
    }

    #[test]
    fn get_rpn_empty_parens_test_2(){
        let test = "3 + 4 ( )";
        assert_eq!(get_rpn(test).unwrap(), vec!["3", "4", "+"]);
    }

    #[test]
    fn get_rpn_invalid_opening_parens_test(){
        let test = "( ( 3 + 4 )";
        assert_eq!(get_rpn(test).unwrap_err(), String::from("Invalid Opening Parenthesis"));
    }

    #[test]
    fn get_rpn_invalid_opening_parens_test_2(){
        let test = "( 3 + 4 ( ( )";
        assert_eq!(get_rpn(test).unwrap_err(), String::from("Invalid Opening Parenthesis"));
    }

    #[test]
    fn get_rpn_test_1(){
        let test = "3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3";
        let ans = vec!["3", "4", "2", "*", "1", "5", "-", "2", "3", "^", "^", "/", "+"];
        assert_eq!(get_rpn(test).unwrap(),ans);
    }

    #[test]
    fn get_rpn_test_2(){
        let test = "3 + 4 * ( 2 - 1 )";
        let ans = vec!["3", "4", "2", "1", "-", "*", "+"];
        assert_eq!(get_rpn(test).unwrap(),ans);
    }

    #[test]
    fn get_rpn_test_trig(){
        let test = "sin ( max ( 2 , 3 ) / 3 * π )";
        let ans = vec!["2", "3", "max", "3", "/", "π", "*", "sin"];
        assert_eq!(get_rpn(test).unwrap(),ans);
    }

    #[test]
    fn eval_rpn_test_1(){
        let test = "3 + 4 * ( 2 - 1 )";
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, f32::NAN).unwrap();
        assert_eq!(ans, 7_f32);
    }

    #[test]
    fn eval_rpn_test_2(){
        let test = "3 + 4 * 2 - 1";
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, f32::NAN).unwrap();
        assert_eq!(ans, 10_f32);
    }

    #[test]
    fn eval_rpn_test_3(){
        let test = "y = 3 + 4 * ( 2 - x )";
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, 1_f32).unwrap();
        assert_eq!(ans, 7_f32);
    }

    #[test]
    fn eval_rpn_test_4(){
        let test = "y = x ^ 2 + x + 3";
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, 2_f32).unwrap();
        assert_eq!(ans, 9_f32);
    }

    #[test]
    fn eval_rpn_test_trig_2(){
        let test = "sin π";
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, f32::NAN).unwrap();
        assert!(is_close(ans, 0_f32));
    }

    #[test]
    fn eval_rpn_test_trig_3(){
        let test = " sin ( π ) / 2";
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, f32::NAN).unwrap();
        assert!(is_close(ans, 0_f32));
    }

    #[test]
    fn eval_rpn_test_trig_4(){
        let test = "sin ( π / 2 )";
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, f32::NAN).unwrap();
        assert!(is_close(ans, 1_f32));
    }

    #[test]
    fn eval_rpn_test_trig_5(){
        let test = "cos ( π ) / 2";
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, f32::NAN).unwrap();
        assert!(is_close(ans, -0.5f32));
    }

    #[test]
    fn eval_rpn_test_trig_6(){
        let test = "tan ( π ) + cos ( π + π ) + sin ( 2 * π )";
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, f32::NAN).unwrap();
        assert!(is_close(ans, 1_f32));
    }

    #[test]
    fn eval_rpn_test_trig_max(){
        let test = "tan ( π ) + max ( 0 , π ) + sin ( 2 * π )";
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, f32::NAN).unwrap();
        assert!(is_close(ans, PI));
    }

    #[test]
    fn eval_rpn_test_trig_max_2(){
        let test = "max ( sin ( π ) , max ( ( 2 ^ 3 ) , 6 ) )";
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, f32::NAN).unwrap();
        assert!(is_close(ans, 8_f32));
    }

    #[test]
    fn eval_rpn_test_abs(){
        let test = "abs ( 2 - 3 ^ 2 )";
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, f32::NAN).unwrap();
        assert_eq!(ans, 7_f32);
    }

    #[test]
    fn eval_rpn_test_abs_2(){
        let test = "abs ( 2 * 3 - 3 ^ 2 )";
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, f32::NAN).unwrap();
        assert_eq!(ans, 3_f32);
    }

    #[test]
    fn eval_rpn_test_sqrt(){
        let test = "sqrt 1764";
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, f32::NAN).unwrap();
        assert_eq!(ans, 42_f32);
    }

    #[test]
    fn eval_rpn_test_min(){
        let test = "min ( max ( 5 , 8 ) , max ( 7 , 9 ) )";
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, f32::NAN).unwrap();
        assert_eq!(ans, 8_f32);
    }

    #[test]
    fn eval_rpn_test_ln(){
        let test = "ln e";
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, f32::NAN).unwrap();
        assert!(is_close(ans, 1_f32));
    }

    #[test]
    fn eval_rpn_test_log(){
        let test = "log_10 10";
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, f32::NAN).unwrap();
        assert!(is_close(ans, 1_f32));
    }

    #[test]
    fn eval_rpn_test_log_add(){
        let test = "log_10 ( 10 ) + log_10 10";
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, f32::NAN).unwrap();
        println!("{}", ans);
        assert!(is_close(ans, 2_f32));
    }

    #[test]
    fn eval_rpn_test_log_add_2(){
        let test = "log_10 ( 10 ) + log_10 ( 5 + 5 )";
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, f32::NAN).unwrap();
        println!("{}", ans);
        assert!(is_close(ans, 2_f32));
    }

    #[test]
    fn eval_rpn_test_log_base_7(){
        let test = "log_7 49";
        let rpn = get_rpn(test).unwrap();
        let ans = eval_rpn(&rpn, f32::NAN).unwrap();
        assert!(is_close(ans, 2_f32));
    }
}