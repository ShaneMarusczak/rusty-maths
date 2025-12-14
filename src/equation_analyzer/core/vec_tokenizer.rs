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
                // Check if this is binary minus (subtraction) or unary minus (negation)
                if s.previous_match(&[_E, _Pi, Number, CloseParen, X, Factorial]) {
                    // Previous token was an operand, so this is binary subtraction
                    s.add_token(Minus);
                } else {
                    // Previous token was an operator, '(', or start of input - this is unary negation
                    s.add_token(UnaryMinus);
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
