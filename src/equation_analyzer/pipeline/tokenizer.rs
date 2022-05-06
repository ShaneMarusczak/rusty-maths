use crate::equation_analyzer::structs::token::{Token, TokenType::*};
use crate::equation_analyzer::structs::tokenizer_state::{Tokenizer, TokenizerState};
use crate::utilities::{is_alpha, is_dig};

pub(crate) fn get_tokens(eq: &str) -> Result<Vec<Token>, String> {
    if eq.is_empty() {
        return Err(String::from("Invalid equation supplied"));
    }
    let mut state = TokenizerState {
        tokens: Vec::with_capacity(eq.len()),
        start: 0,
        current: 0,
        eq,
    };

    while !state.at_end() {
        state.start = state.current;
        let c = state.advance()?;
        match c {
            ' ' | '\r' | '\t' => (),
            'y' => state.add_token(Y, "y"),
            '=' => state.add_token(Equal, "="),
            ',' => state.add_token(Comma, ","),

            //TODO: PI IS BUSTED!! THIS CHAR IS TOO BIG AND THE SCANNER GETS OUT OF SYNC
            // 'π' => {
            //     add_token(_Pi, "π", &mut tokens);
            //     current += 1;
            // }
            'e' => state.add_token(_E, "e"),
            '*' => state.add_token(Star, "*"),
            '/' => state.add_token(Slash, "/"),
            '+' => state.add_token(Plus, "+"),
            '%' => {
                if state.peek()? == '%' {
                    state.advance()?;
                    state.add_token(Modulo, "%");
                } else {
                    state.add_token(Percent, "%%");
                }
            },

            //negative pi or e?
            '-' => {
                if state.previous_match(&[_E, Number, CloseParen, X]) {
                    state.add_token(Minus, "-");
                } else {
                    if state.peek()? == 'e' {
                        state.advance()?;
                        state.add_token(NegE, "-e");
                    } else if state.peek()? == 'π' {
                        state.advance()?;
                        state.add_token(NegPi, "-π");
                    } else if is_dig(state.peek()?) {
                        state.digit()?;
                        if state.peek()? != 'x' {
                            state.add_token(Number, &eq[state.start..state.current]);
                        } else {
                            let coefficient = eq[state.start..state.current].to_owned();
                            //consume the x
                            state.advance()?;
                            state.take_x(coefficient)?;
                        }
                    } else if state.peek()? == 'x' {
                        let coefficient = String::from("-1");
                        state.advance()?;
                        state.take_x(coefficient)?;
                    }
                }
            }

            '(' => state.add_token(OpenParen, "("),
            ')' => state.add_token(CloseParen, ")"),
            '^' => state.add_token(Power, "^"),
            'x' => {
                let coefficient = String::from("1");
                state.take_x(coefficient)?;
            }
            _ => {
                if is_dig(c) {
                    state.digit()?;
                    if state.peek()? != 'x' {
                        let test = &eq[state.start..state.current];
                        state.add_token(Number, test);
                    } else {
                        let coefficient = eq[state.start..state.current].to_owned();
                        //consume the x
                        state.advance()?;
                        state.take_x(coefficient)?;
                    }
                } else if is_alpha(c) {
                    while is_alpha(state.peek()?) {
                        state.advance()?;
                    }
                    let name = &eq[state.start..state.current];

                    if state.peek()? == '_' {
                        if name != "log" {
                            return Err(format!("Invalid input at character {}", state.start));
                        }
                        //consume the _
                        state.advance()?;

                        if is_dig(state.peek()?) {
                            state.digit()?;
                        } else {
                            return Err(String::from("Invalid use of log"));
                        }
                    }
                    if state.peek()? != '(' {
                        return Err(format!("Invalid input at character {}", state.start));
                    }

                    //consume the (
                    state.advance()?;
                    let literal = &eq[state.start..state.current];
                    match name {
                        "sin" => state.add_token(Sin, literal),
                        "cos" => state.add_token(Cos, literal),
                        "tan" => state.add_token(Tan, literal),
                        "max" => state.add_token(Max, literal),
                        "abs" => state.add_token(Abs, literal),
                        "sqrt" => state.add_token(Sqrt, literal),
                        "min" => state.add_token(Min, literal),
                        "ln" => state.add_token(Ln, literal),
                        "log" => state.add_token(Log, literal),
                        _ => return Err(format!("Invalid input at character {}", state.start)),
                    }
                } else {
                    return Err(format!("Invalid input at character {}", state.current));
                }
            }
        }
    }
    state.tokens.push(Token {
        token_type: End,
        literal: "end".to_owned(),
    });
    Ok(state.tokens)
}
