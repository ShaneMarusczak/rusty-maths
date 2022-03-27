use crate::equation_analyzer::structs::token::{Token, TokenType, TokenType::*};

pub(crate) fn get_tokens(eq: &str) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::with_capacity(eq.len());

    let mut current: usize = 0;
    let mut start: usize;

    while !at_end(current, eq) {
        start = current;
        let c = advance(eq, &mut current)?;
        match c {
            ' ' | '\r' | '\t' => (),
            'y' => add_token(Y, "y", &mut tokens),
            '=' => add_token(Equal, "=", &mut tokens),
            ',' => add_token(Comma, ",", &mut tokens),
            'π' => {
                add_token(_Pi, "π", &mut tokens);
                current += 1;
            }
            'e' => add_token(_E, "e", &mut tokens),
            '*' => add_token(Star, "*", &mut tokens),
            '/' => add_token(Slash, "/", &mut tokens),
            '+' => add_token(Plus, "+", &mut tokens),

            //negative pi or e?
            '-' => {
                if peek(eq, current)? == 'e' {
                    advance(eq, &mut current)?;
                    add_token(NegE, "-e", &mut tokens);
                } else if peek(eq, current)? == 'π' {
                    advance(eq, &mut current)?;
                    add_token(NegPi, "-π", &mut tokens);
                } else if is_dig(peek(eq, current)?) {
                    digit(eq, &mut current)?;
                    if peek(eq, current)? != 'x' {
                        add_token(Number, &eq[start..current], &mut tokens);
                    } else {
                        let coefficient = eq[start..current].to_owned();
                        //consume the x
                        advance(eq, &mut current)?;
                        take_x(eq, &mut current, &mut tokens, coefficient)?;
                    }
                } else if peek(eq, current)? == 'x' {
                    let coefficient = String::from("-1");
                    advance(eq, &mut current)?;
                    take_x(eq, &mut current, &mut tokens, coefficient)?;
                } else {
                    add_token(Minus, "-", &mut tokens);
                }
            }

            '(' => add_token(OpenParen, "(", &mut tokens),
            ')' => add_token(CloseParen, ")", &mut tokens),
            '^' => add_token(Power, "^", &mut tokens),
            'x' => {
                let coefficient = String::from("1");
                take_x(eq, &mut current, &mut tokens, coefficient)?;
            }
            _ => {
                if is_dig(c) {
                    digit(eq, &mut current)?;
                    if peek(eq, current)? != 'x' {
                        let test = &eq[start..current];
                        add_token(Number, test, &mut tokens);
                    } else {
                        let coefficient = eq[start..current].to_owned();
                        //consume the x
                        advance(eq, &mut current)?;
                        take_x(eq, &mut current, &mut tokens, coefficient)?;
                    }
                } else if is_alpha(c) {
                    while is_alpha(peek(eq, current)?) {
                        advance(eq, &mut current)?;
                    }
                    let name = &eq[start..current];

                    if peek(eq, current)? == '_' {
                        if name != "log" {
                            return Err(format!("Invalid input at character {}", start));
                        }
                        //consume the _
                        advance(eq, &mut current)?;

                        if is_dig(peek(eq, current)?) {
                            digit(eq, &mut current)?;
                        } else {
                            return Err(String::from("Invalid use of log"));
                        }
                    }
                    if peek(eq, current)? != '(' {
                        return Err(format!("Invalid input at character {}", start));
                    }

                    //consume the (
                    advance(eq, &mut current)?;

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
                        _ => return Err(format!("Invalid input at character {}", start)),
                    }
                }
            }
        }
    }
    tokens.push(Token {
        token_type: End,
        literal: "end".to_owned(),
    });
    Ok(tokens)
}

fn take_x(
    eq: &str,
    mut current: &mut usize,
    mut tokens: &mut Vec<Token>,
    coefficient: String,
) -> Result<(), String> {
    let pow_string;

    if peek(eq, *current)? == '^' {
        let pow_start = *current;
        power(eq, &mut current)?;
        pow_string = &eq[pow_start..*current];
    } else {
        pow_string = "^1";
    }

    let final_literal = coefficient + "x" + pow_string;

    add_token(X, &final_literal, &mut tokens);
    Ok(())
}

fn power(eq: &str, mut current: &mut usize) -> Result<(), String> {
    if is_dig(peek_next(eq, *current)?) {
        //consume the ^
        advance(eq, &mut current)?;
        digit(eq, &mut current)?;
        Ok(())
    } else {
        return Err(String::from("Invalid power"));
    }
}

fn digit(eq: &str, mut current: &mut usize) -> Result<(), String> {
    while is_dig(peek(eq, *current)?) {
        advance(eq, &mut current)?;
    }
    if peek(eq, *current)? == '.' && is_dig(peek_next(eq, *current)?) {
        //consume the .
        advance(eq, &mut current)?;
        while is_dig(peek(eq, *current)?) {
            advance(eq, &mut current)?;
        }
    }
    Ok(())
}

fn is_dig(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')
}

fn advance(eq: &str, current: &mut usize) -> Result<char, String> {
    let c = eq.chars().nth(*current);
    *current += 1;
    return if c.is_some() {
        Ok(c.unwrap())
    } else {
        Err(format!("Invalid Input at {}", current))
    };
}

fn peek(eq: &str, current: usize) -> Result<char, String> {
    if at_end(current, eq) {
        return Ok('\0');
    };
    let c = eq.chars().nth(current);
    return if c.is_some() {
        Ok(c.unwrap())
    } else {
        Err(format!("Invalid Input at {}", current))
    };
}

fn peek_next(eq: &str, current: usize) -> Result<char, String> {
    if at_end(current + 1, eq) {
        return Ok('\0');
    };
    let c = eq.chars().nth(current + 1);
    return if c.is_some() {
        Ok(c.unwrap())
    } else {
        Err(format!("Invalid Input at {}", current))
    };
}

fn at_end(current: usize, eq: &str) -> bool {
    current >= eq.chars().count()
}

fn add_token(token_type: TokenType, literal: &str, tokens: &mut Vec<Token>) {
    let token = Token {
        token_type,
        literal: literal.to_owned(),
    };
    tokens.push(token);
}
