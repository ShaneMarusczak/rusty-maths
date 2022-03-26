use crate::equation_analyzer::structs::token::{Token, TokenType, TokenType::*};

fn get_tokens(eq: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::with_capacity(eq.len());

    let mut current: usize = 0;
    let mut start: usize = 0;

    while !at_end(current, eq) {
        start = current;
        let c = advance(eq, &mut current);
        match c {
            ' ' | '\r' | '\t' => (),
            'y' => add_token(Y, "y", &mut tokens),
            '=' => add_token(Equal, "=", &mut tokens),
            ',' => add_token(Comma, ",", &mut tokens),
            'π' => add_token(_Pi, "π", &mut tokens),
            'e' => add_token(_E, "e", &mut tokens),
            '*' => add_token(Star, "*", &mut tokens),
            '/' => add_token(Slash, "/", &mut tokens),
            '+' => add_token(Plus, "+", &mut tokens),
            '-' => add_token(Minus, "-", &mut tokens),
            '(' => add_token(OpenParen, "(", &mut tokens),
            ')' => add_token(CloseParen, ")", &mut tokens),
            '^' => add_token(Power, "^", &mut tokens),
            'x' => {
                let coefficient = String::from("1");
                take_x(eq, &mut current, &mut tokens, coefficient)
            }
            _ => {
                if is_dig(c) {
                    digit(eq, &mut current);
                    if peek(eq, current) != 'x' {
                        add_token(Number, &eq[start..current], &mut tokens);
                    } else {
                        let coefficient = eq[start..current].to_owned();
                        //consume the x
                        advance(eq, &mut current);
                        take_x(eq, &mut current, &mut tokens, coefficient);
                    }
                } else if is_alpha(c) {
                    while is_alpha(peek(eq, current)) {
                        advance(eq, &mut current);
                    }
                    let name = &eq[start..current];

                    if peek(eq, current) == '_' {
                        if name != "log" {
                            panic!("Invalid function");
                        }
                        //consume the _
                        advance(eq, &mut current);

                        if is_dig(peek(eq, current)) {
                            digit(eq, &mut current);
                        } else {
                            panic!("Invalid use of log");
                        }
                    }
                    if peek(eq, current) != '(' {
                        panic!("Invalid function");
                    }

                    //consume the (
                    advance(eq, &mut current);

                    match name {
                        "sin" => add_token(Sin, &eq[start..current], &mut tokens),
                        "cos" => add_token(Cos, &eq[start..current], &mut tokens),
                        "tan" => add_token(Tan, &eq[start..current], &mut tokens),
                        "max" => add_token(Max, &eq[start..current], &mut tokens),
                        "abs" => add_token(Abs, &eq[start..current], &mut tokens),
                        "sqrt" => add_token(Sqrt, &eq[start..current], &mut tokens),
                        "min" => add_token(Min, &eq[start..current], &mut tokens),
                        "ln" => add_token(Ln, &eq[start..current], &mut tokens),
                        "log" => add_token(Log, &eq[start..current], &mut tokens),
                        _ => panic!("invalid function"),
                    }
                }
            }
        }
    }
    tokens.push(Token {
        token_type: End,
        literal: "end".to_owned(),
    });
    tokens
}

fn take_x(eq: &str, mut current: &mut usize, mut tokens: &mut Vec<Token>, coefficient: String) {
    let mut pow_string;

    if peek(eq, *current) == '^' {
        let pow_start = *current;
        power(eq, &mut current);
        pow_string = &eq[pow_start..*current];
    } else {
        pow_string = "^1";
    }

    let final_literal = coefficient + "x" + pow_string;

    add_token(X, &final_literal, &mut tokens);
}

fn power(eq: &str, mut current: &mut usize) {
    if is_dig(peek_next(eq, *current)) {
        //consume the ^
        advance(eq, &mut current);
        digit(eq, &mut current);
    } else {
        panic!("Invalid power");
    }
}

fn digit(eq: &str, mut current: &mut usize) {
    while is_dig(peek(eq, *current)) {
        advance(eq, &mut current);
    }
    if peek(eq, *current) == '.' && is_dig(peek_next(eq, *current)) {
        //consume the .
        advance(eq, &mut current);
        while is_dig(peek(eq, *current)) {
            advance(eq, &mut current);
        }
    }
}

fn is_dig(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')
}

fn advance(eq: &str, mut current: &mut usize) -> char {
    let c = eq
        .chars()
        .nth(*current)
        .unwrap_or_else(|| panic!("Invalid Input"));
    *current += 1;
    c
}

fn peek(eq: &str, current: usize) -> char {
    if at_end(current, eq) {
        return '\0';
    };
    eq.chars()
        .nth(current)
        .unwrap_or_else(|| panic!("Invalid Input"))
}

fn peek_next(eq: &str, current: usize) -> char {
    if at_end(current + 1, eq) {
        return '\0';
    };
    eq.chars()
        .nth(current + 1)
        .unwrap_or_else(|| panic!("Invalid Input"))
}

fn match_char(c: char, eq: &str, current: usize) -> bool {
    peek(eq, current) == c
}

fn at_end(current: usize, eq: &str) -> bool {
    current >= eq.len()
}

fn add_token(token_type: TokenType, literal: &str, tokens: &mut Vec<Token>) {
    let token = Token {
        token_type,
        literal: literal.to_owned(),
    };
    tokens.push(token);
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
    fn test_1() {
        let eq = "y = 32.2";

        let ans = vec![
            get_token(Y, "y"),
            get_token(Equal, "="),
            get_token(Number, "32.2"),
            get_token(End, "end"),
        ];

        assert_eq!(get_tokens(eq), ans);
    }

    #[test]
    fn test_2() {
        let eq = "y=e +47- x";
        let ans = vec![
            get_token(Y, "y"),
            get_token(Equal, "="),
            get_token(_E, "e"),
            get_token(Plus, "+"),
            get_token(Number, "47"),
            get_token(Minus, "-"),
            get_token(X, "1x^1"),
            get_token(End, "end"),
        ];

        assert_eq!(get_tokens(eq), ans);
    }

    #[test]
    fn test_3() {
        let eq = "y=3x";
        let ans = vec![
            get_token(Y, "y"),
            get_token(Equal, "="),
            get_token(X, "3x^1"),
            get_token(End, "end"),
        ];
        assert_eq!(get_tokens(eq), ans);
    }

    #[test]
    fn test_4() {
        let eq = "y= 3x^2 +x";
        let ans = vec![
            get_token(Y, "y"),
            get_token(Equal, "="),
            get_token(X, "3x^2"),
            get_token(Plus, "+"),
            get_token(X, "1x^1"),
            get_token(End, "end"),
        ];
        assert_eq!(get_tokens(eq), ans);
    }

    #[test]
    fn test_5() {
        let eq = "y= sin(3x+ 2 )";
        let ans = vec![
            get_token(Y, "y"),
            get_token(Equal, "="),
            get_token(Sin, "sin("),
            get_token(X, "3x^1"),
            get_token(Plus, "+"),
            get_token(Number, "2"),
            get_token(CloseParen, ")"),
            get_token(End, "end"),
        ];
        assert_eq!(get_tokens(eq), ans);
    }

    #[test]
    fn test_6() {
        let eq = "y= ln(3x+ 2 ) * ( 3-2)/ 6";
        let ans = vec![
            get_token(Y, "y"),
            get_token(Equal, "="),
            get_token(Ln, "ln("),
            get_token(X, "3x^1"),
            get_token(Plus, "+"),
            get_token(Number, "2"),
            get_token(CloseParen, ")"),
            get_token(Star, "*"),
            get_token(OpenParen, "("),
            get_token(Number, "3"),
            get_token(Minus, "-"),
            get_token(Number, "2"),
            get_token(CloseParen, ")"),
            get_token(Slash, "/"),
            get_token(Number, "6"),
            get_token(End, "end"),
        ];
        assert_eq!(get_tokens(eq), ans);
    }

    #[test]
    fn test_7() {
        let eq = "y=log_3(x )";
        let ans = vec![
            get_token(Y, "y"),
            get_token(Equal, "="),
            get_token(Log, "log_3("),
            get_token(X, "1x^1"),
            get_token(CloseParen, ")"),
            get_token(End, "end"),
        ];
        assert_eq!(get_tokens(eq), ans);
    }

    #[test]
    fn test_8() {
        let eq = "y=3 ^x";
        let ans = vec![
            get_token(Y, "y"),
            get_token(Equal, "="),
            get_token(Number, "3"),
            get_token(Power, "^"),
            get_token(X, "1x^1"),
            get_token(End, "end"),
        ];
        assert_eq!(get_tokens(eq), ans);
    }
}
