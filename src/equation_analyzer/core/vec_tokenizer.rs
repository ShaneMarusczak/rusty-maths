use crate::equation_analyzer::structs::token::{Token, TokenType::*};
use crate::equation_analyzer::structs::tokenizer_state::{Tokenizer, TokenizerState};
use crate::utilities::get_str_section;

/// Tokenizes a mathematical equation string into a vector of tokens.
///
/// # Arguments
/// * `eq` - The equation string to tokenize
///
/// # Returns
/// * `Ok(Vec<Token>)` - A vector of tokens representing the equation
/// * `Err(String)` - An error message if tokenization fails
///
/// # Supported Operations
/// - Arithmetic: +, -, *, /, ^, %, %%
/// - Functions: sin, cos, tan, asin, acos, atan, abs, sqrt, ln, log_N
/// - Statistical: min, max, avg, med, mode
/// - Constants: e, π
/// - Factorial: !
/// - Parentheses: (, )
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
            '-' => {
                if s.previous_match(&[_E, _Pi, Number, CloseParen, X, Factorial]) {
                    s.add_token(Minus);
                } else if s.peek()? == 'e' {
                    s.advance()?;
                    s.add_token(NegE);
                } else if s.peek()? == 'π' {
                    s.advance()?;
                    s.add_token(NegPi);
                } else if s.peek()?.is_ascii_digit() {
                    // Emit -1 * NUMBER to preserve operator precedence
                    // This ensures -2^2 evaluates as -(2^2) = -4, not (-2)^2 = 4
                    s.digit()?;
                    // Check if we need to wrap in parentheses (after operators with precedence >= 3)
                    // We wrap after: Power(4), Star(3), Slash(3), Modulo(3), Percent(3)
                    // We don't wrap after Plus(2) or Minus(2) because * has higher precedence
                    let prev_is_high_prec = s.tokens.last().is_some_and(|t| matches!(
                        t.token_type,
                        Power | Star | Slash | Modulo | Percent
                    ));
                    let needs_parens = prev_is_high_prec;

                    if s.peek()? != 'x' {
                        // Skip the minus, just get the digits
                        let literal = get_str_section(eq, s.start + 1, s.current);
                        let num = literal.parse::<f32>()
                            .map_err(|_| format!("Invalid number: {}", literal))?;
                        // If after binary operator, wrap in parentheses: 3/(-1 * 2)
                        if needs_parens {
                            s.add_token(OpenParen);
                            s.add_token_n(Number, -1.0, 0.0);
                            s.add_token(Star);
                            s.add_token_n(Number, num, 0.0);
                            s.add_token(CloseParen);
                        } else {
                            // Normal case: -1 * NUMBER
                            s.add_token_n(Number, -1.0, 0.0);
                            s.add_token(Star);
                            s.add_token_n(Number, num, 0.0);
                        }
                    } else {
                        // For -Nx, emit -1 * N * x (with parens if needed)
                        let coefficient = get_str_section(eq, s.start + 1, s.current);
                        let coef_val = coefficient.parse::<f32>()
                            .map_err(|_| format!("Invalid coefficient: {}", coefficient))?;
                        s.advance()?; // consume the x
                        if needs_parens {
                            s.add_token(OpenParen);
                        }
                        s.add_token_n(Number, -1.0, 0.0);
                        s.add_token(Star);
                        s.add_token_n(Number, coef_val, 0.0);
                        s.add_token(Star);
                        s.add_token_n(X, 1.0, 1.0);
                        if needs_parens {
                            s.add_token(CloseParen);
                        }
                    }
                } else if s.peek()? == 'x' {
                    // For -x, emit -1 * x (with parens if needed)
                    s.advance()?; // consume the x
                    let prev_is_high_prec = s.tokens.last().is_some_and(|t| matches!(
                        t.token_type,
                        Power | Star | Slash | Modulo | Percent
                    ));
                    let needs_parens = prev_is_high_prec;
                    if needs_parens {
                        s.add_token(OpenParen);
                    }
                    s.add_token_n(Number, -1.0, 0.0);
                    s.add_token(Star);
                    s.add_token_n(X, 1.0, 1.0);
                    if needs_parens {
                        s.add_token(CloseParen);
                    }
                } else if s.peek()? == '(' || s.peek()?.is_alphabetic() || s.peek()? == '-' {
                    //-(5) or -sqrt(4) or --2
                    s.add_token_n(Number, -1.0, 0.0);
                    s.add_token(Star);
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
                if c.is_ascii_digit() {
                    s.digit()?;
                    if s.peek()? != 'x' {
                        let literal = get_str_section(eq, s.start, s.current);

                        let num = literal.parse::<f32>()
                            .map_err(|_| format!("Invalid number: {}", literal))?;
                        s.add_token_n(Number, num, 0.0);
                    } else {
                        let coefficient = get_str_section(eq, s.start, s.current);
                        //consume the x
                        s.advance()?;
                        s.take_x(coefficient)?;
                    }
                } else if c.is_alphabetic() {
                    while s.peek()?.is_alphabetic() {
                        s.advance()?;
                    }
                    let name = get_str_section(eq, s.start, s.current);

                    if s.peek()? == '_' {
                        if name != "log" {
                            return Err(format!("Invalid input at character {}", s.current));
                        }
                        //consume the _
                        s.advance()?;

                        if s.peek()?.is_ascii_digit() {
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
                        "asin" => s.add_token(Asin),
                        "acos" => s.add_token(Acos),
                        "atan" => s.add_token(Atan),
                        "max" => s.add_token(Max),
                        "abs" => s.add_token(Abs),
                        "sqrt" => s.add_token(Sqrt),
                        "min" => s.add_token(Min),
                        "ln" => s.add_token(Ln),
                        "avg" => s.add_token(Avg),
                        "med" => s.add_token(Med),
                        "mode" => s.add_token(Mode),
                        "ch" => s.add_token(Choice),
                        "log" => {
                            let mut literal = get_str_section(eq, s.start, s.current);
                            literal.pop();
                            let base_str = literal.split('_').nth(1)
                                .ok_or_else(|| String::from("Invalid log format: missing base"))?;
                            let base = base_str.parse::<f32>()
                                .map_err(|_| format!("Invalid log base: {}", base_str))?;
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
