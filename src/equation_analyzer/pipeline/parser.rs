use crate::equation_analyzer::structs::operands::{get_operator, Operand};
use crate::equation_analyzer::structs::token::{Token, TokenType};

pub(crate) fn parse(tokens: Vec<Token>) -> Result<Vec<String>, String> {
    let mut operator_stack: Vec<Operand> = Vec::with_capacity(tokens.len());

    let mut output: Vec<String> = Vec::with_capacity(tokens.len());

    let mut paren_depth = 0;

    let mut found_end = false;

    for token in tokens {
        match token.token_type {
            TokenType::Y | TokenType::Equal | TokenType::Comma => continue,

            TokenType::_Pi | TokenType::_E | TokenType::NegPi | TokenType::NegE => {
                output.push(token.literal);
            }
            TokenType::Sin
            | TokenType::Cos
            | TokenType::Tan
            | TokenType::Max
            | TokenType::Abs
            | TokenType::Sqrt
            | TokenType::Min
            | TokenType::Ln
            | TokenType::Log
            | TokenType::OpenParen => {
                paren_depth += 1;

                operator_stack.push(get_operator(&token.literal));
            }

            TokenType::CloseParen => {
                paren_depth -= 1;

                if paren_depth < 0 {
                    return Err("Invalid closing parenthesis".to_string());
                }

                while !operator_stack.is_empty() && !operator_stack.last().unwrap().paren_opener {
                    let op = operator_stack.pop().unwrap();
                    output.push(op.token);
                }

                if operator_stack.last().unwrap().token == "(" {
                    operator_stack.pop();
                    continue;
                }

                if !operator_stack.is_empty() && operator_stack.last().unwrap().is_func {
                    let func = operator_stack.pop().unwrap();
                    output.push(func.token);
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
                let o_1 = get_operator(&token.literal);
                while !operator_stack.is_empty()
                    // && operator_stack.last().unwrap().token != "("
                    && !operator_stack.last().unwrap().paren_opener
                    && (operator_stack.last().unwrap().prec > o_1.prec
                    || (operator_stack.last().unwrap().prec == o_1.prec && o_1.assoc == "l"))
                {
                    let o_2_new = operator_stack.pop().unwrap();
                    output.push(o_2_new.token);
                }

                operator_stack.push(o_1);
            }

            TokenType::Number | TokenType::X => output.push(token.literal),
            TokenType::End => {
                found_end = true;
            }
        }
    }

    while !operator_stack.is_empty() {
        let op = operator_stack.pop().unwrap();
        if op.token == "(" {
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
