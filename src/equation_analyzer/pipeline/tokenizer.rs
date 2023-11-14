use crate::equation_analyzer::structs::token::{Token, TokenType::*};
use crate::equation_analyzer::structs::tokenizer_state::{Tokenizer, TokenizerState};
use crate::utilities::{get_str_section, is_alpha, is_dig};

pub(crate) fn get_tokens(eq: &str) -> Result<Vec<Token>, String> {
    if eq.is_empty() {
        return Err(String::from("Invalid equation supplied"));
    }

    let mut s = TokenizerState {
        tokens: Vec::with_capacity(eq.chars().count()),
        start: 0,
        current: 0,
        eq,
    };

    while !s.at_end() {
        s.start = s.current;
        let c = s.advance()?;
        match c {
            ' ' | '\r' | '\t' => (),
            'y' => s.add_token(Y),
            '=' => s.add_token(Equal),
            ',' => s.add_token(Comma),
            'π' => s.add_token(_Pi),
            'e' => s.add_token(_E),
            '*' => s.add_token(Star),
            '/' => s.add_token(Slash),
            '+' => s.add_token(Plus),
            '!' => s.add_token(Factorial),
            '%' => {
                if s.peek()? == '%' {
                    s.advance()?;
                    s.add_token(Modulo);
                } else {
                    s.add_token(Percent);
                }
            }

            //negative pi or e?
            '-' => {
                if s.previous_match(&[_E, _Pi, Number, CloseParen, X, Factorial]) {
                    s.add_token(Minus);
                } else if s.peek()? == 'e' {
                    s.advance()?;
                    s.add_token(NegE);
                } else if s.peek()? == 'π' {
                    s.advance()?;
                    s.add_token(NegPi);
                } else if is_dig(s.peek()?) {
                    s.digit()?;
                    if s.peek()? != 'x' {
                        let literal = get_str_section(eq, s.start, s.current);

                        s.add_token_n(Number, literal.parse().unwrap(), 0.0);
                    } else {
                        let coefficient = get_str_section(eq, s.start, s.current);
                        //consume the x
                        s.advance()?;
                        s.take_x(coefficient)?;
                    }
                } else if s.peek()? == 'x' {
                    let coefficient = String::from("-1");
                    s.advance()?;
                    s.take_x(coefficient)?;
                }
            }
            '(' => s.add_token(OpenParen),
            ')' => s.add_token(CloseParen),
            '^' => s.add_token(Power),
            'x' => {
                let coefficient = String::from("1");
                s.take_x(coefficient)?;
            }
            _ => {
                if is_dig(c) {
                    s.digit()?;
                    if s.peek()? != 'x' {
                        let literal = get_str_section(eq, s.start, s.current);

                        s.add_token_n(Number, literal.parse().unwrap(), 0.0);
                    } else {
                        let coefficient = get_str_section(eq, s.start, s.current);
                        //consume the x
                        s.advance()?;
                        s.take_x(coefficient)?;
                    }
                } else if is_alpha(c) {
                    while is_alpha(s.peek()?) {
                        s.advance()?;
                    }
                    let name = get_str_section(eq, s.start, s.current);

                    if s.peek()? == '_' {
                        if name != "log" {
                            return Err(format!("Invalid input at character {}", s.current));
                        }
                        //consume the _
                        s.advance()?;

                        if is_dig(s.peek()?) {
                            s.digit()?;
                        } else {
                            return Err(String::from("Invalid use of log"));
                        }
                    }
                    if s.peek()? != '(' {
                        return Err(format!("Invalid input at character {}", s.start));
                    }

                    //consume the (
                    s.advance()?;

                    match name.as_str() {
                        "sin" => s.add_token(Sin),
                        "cos" => s.add_token(Cos),
                        "tan" => s.add_token(Tan),
                        "max" => s.add_token(Max),
                        "abs" => s.add_token(Abs),
                        "sqrt" => s.add_token(Sqrt),
                        "min" => s.add_token(Min),
                        "ln" => s.add_token(Ln),
                        "avg" => s.add_token(Avg),
                        "log" => {
                            let mut literal = get_str_section(eq, s.start, s.current);
                            literal.pop();
                            let base = literal.split('_').nth(1).unwrap().parse::<f32>().unwrap();
                            s.add_token_n(Log, base, 0.0);
                        }
                        _ => return Err(format!("Invalid function name {}", name)),
                    }
                } else {
                    return Err(format!("Invalid input at character {}", s.current));
                }
            }
        }
    }
    s.add_token(End);
    Ok(s.tokens)
}
