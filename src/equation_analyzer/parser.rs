use crate::equation_analyzer::operands::{get_operator, Operand};
use crate::equation_analyzer::structs::token::{Token, TokenType};

fn parse(tokens: Vec<Token>) -> Result<Vec<String>, String> {
    let mut operator_stack: Vec<Operand> = Vec::with_capacity(tokens.len());

    let mut output: Vec<String> = Vec::with_capacity(tokens.len());

    let mut paren_depth = 0;

    for token in tokens {
        match token.token_type {
            TokenType::Y | TokenType::Equal | TokenType::Comma => continue,

            TokenType::_Pi | TokenType::_E => {
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
                    return Err(format!("invalid closing parenthesis at character"));
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
            | TokenType::Power => {
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
            TokenType::End => {}
        }
    }

    while !operator_stack.is_empty() {
        let op = operator_stack.pop().unwrap();
        if op.token == "(" {
            return Err("invalid opening parenthesis".to_string());
        }
        output.push(op.token);
    }
    if paren_depth != 0 {
        return Err("invalid function".to_string());
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_token(t_t: TokenType, lit: &str) -> Token {
        Token {
            token_type: t_t,
            literal: lit.to_owned(),
        }
    }

    #[test]
    fn parse_test_1() {
        //y = 3 + 4 * ( 2 - 1 )
        let test = vec![
            get_token(TokenType::Y, "y"),
            get_token(TokenType::Equal, "="),
            get_token(TokenType::Number, "3"),
            get_token(TokenType::Plus, "+"),
            get_token(TokenType::Number, "4"),
            get_token(TokenType::Star, "*"),
            get_token(TokenType::OpenParen, "("),
            get_token(TokenType::Number, "2"),
            get_token(TokenType::Minus, "-"),
            get_token(TokenType::Number, "1"),
            get_token(TokenType::CloseParen, ")"),
            get_token(TokenType::End, "end"),
        ];
        let ans = vec!["3", "4", "2", "1", "-", "*", "+"];

        assert_eq!(parse(test).unwrap(), ans);
    }

    #[test]
    fn parse_test_2() {
        //2 ^ x;
        let test = vec![
            get_token(TokenType::Number, "2"),
            get_token(TokenType::Power, "^"),
            get_token(TokenType::X, "1x^1"),
            ];
        assert_eq!(parse(test).unwrap(), vec!["2", "1x^1", "^"]);
    }

    #[test]
    fn parse_test_3() {
        //3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3;
        let test = vec![
            get_token(TokenType::Number, "3"),
            get_token(TokenType::Plus, "+"),
            get_token(TokenType::Number, "4"),
            get_token(TokenType::Star, "*"),
            get_token(TokenType::Number, "2"),
            get_token(TokenType::Slash, "/"),
            get_token(TokenType::OpenParen, "("),
            get_token(TokenType::Number, "1"),
            get_token(TokenType::Minus, "-"),
            get_token(TokenType::Number, "5"),
            get_token(TokenType::CloseParen, ")"),
            get_token(TokenType::Power, "^"),
            get_token(TokenType::Number, "2"),
            get_token(TokenType::Power, "^"),
            get_token(TokenType::Number, "3"),
            get_token(TokenType::End, "end"),
        ];
        let ans = vec![
            "3", "4", "2", "*", "1", "5", "-", "2", "3", "^", "^", "/", "+",
        ];

        assert_eq!(parse(test).unwrap(), ans);

    }

    #[test]
    fn parse_test_4() {
        //"3 ^ 2 + 4 * ( 2 - 1 )";
        let test = vec![
            get_token(TokenType::Number, "3"),
            get_token(TokenType::Power, "^"),
            get_token(TokenType::Number, "2"),
            get_token(TokenType::Plus, "+"),
            get_token(TokenType::Number, "4"),
            get_token(TokenType::Star, "*"),
            get_token(TokenType::OpenParen, "("),
            get_token(TokenType::Number, "2"),
            get_token(TokenType::Minus, "-"),
            get_token(TokenType::Number, "1"),
            get_token(TokenType::CloseParen, ")"),
            get_token(TokenType::End, "end"),
        ];

        let ans = vec!["3", "2", "^", "4", "2", "1", "-", "*", "+"];
        assert_eq!(parse(test).unwrap(), ans);
    }

    #[test]
    fn parse_test_5() {
        //sin( max( ( 2 + 0 ) , 3 ) / ( 3 * π ) )
        let test = vec![
            get_token(TokenType::Sin, "sin("),
            get_token(TokenType::Max, "max("),
            get_token(TokenType::OpenParen, "("),
            get_token(TokenType::Number, "2"),
            get_token(TokenType::Plus, "+"),
            get_token(TokenType::Number, "0"),
            get_token(TokenType::CloseParen, ")"),
            get_token(TokenType::Comma, ","),
            get_token(TokenType::Number, "3"),
            get_token(TokenType::CloseParen, ")"),
            get_token(TokenType::Slash, "/"),
            get_token(TokenType::OpenParen, "("),
            get_token(TokenType::Number, "3"),
            get_token(TokenType::Star, "*"),
            get_token(TokenType::_Pi, "π"),
            get_token(TokenType::CloseParen, ")"),
            get_token(TokenType::CloseParen, ")"),
            get_token(TokenType::End, "end"),
        ];


        let ans = vec!["2", "0", "+", "3", "max(", "3", "π", "*", "/", "sin("];
        assert_eq!(parse(test).unwrap(), ans);

    }
}
