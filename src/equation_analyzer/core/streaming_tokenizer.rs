use crate::equation_analyzer::structs::token::{Token, TokenType};
use std::collections::VecDeque;
use std::iter::Peekable;
use std::str::Chars;

/// A streaming tokenizer that implements Iterator, yielding tokens on demand
pub(crate) struct StreamingTokenizer<'a> {
    chars: Peekable<Chars<'a>>,
    eq: &'a str,
    position: usize,
    start_position: usize,
    previous_token_type: Option<TokenType>,
    finished: bool,
    pending_tokens: VecDeque<Token>,
}

impl<'a> StreamingTokenizer<'a> {
    pub(crate) fn new(eq: &'a str) -> Result<Self, String> {
        if eq.is_empty() {
            return Err(String::from("Invalid equation supplied"));
        }

        Ok(Self {
            chars: eq.chars().peekable(),
            eq,
            position: 0,
            start_position: 0,
            previous_token_type: None,
            finished: false,
            pending_tokens: VecDeque::new(),
        })
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.chars.next();
        if let Some(c) = ch {
            self.position += c.len_utf8();
        }
        ch
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn peek_ahead(&self, n: usize) -> Option<char> {
        self.eq.chars().nth(self.position + n)
    }

    fn get_substring(&self, start: usize, end: usize) -> &'a str {
        &self.eq[start..end]
    }

    fn previous_match(&self, types: &[TokenType]) -> bool {
        self.previous_token_type
            .as_ref()
            .map_or(false, |prev| types.contains(prev))
    }

    fn make_token(&mut self, token_type: TokenType) -> Token {
        let token = Token {
            token_type,
            numeric_value_1: 0.0,
            numeric_value_2: 0.0,
        };
        self.previous_token_type = Some(token_type);
        token
    }

    fn make_token_with_values(&mut self, token_type: TokenType, val1: f32, val2: f32) -> Token {
        let token = Token {
            token_type,
            numeric_value_1: val1,
            numeric_value_2: val2,
        };
        self.previous_token_type = Some(token_type);
        token
    }

    fn scan_digit(&mut self) -> Result<String, String> {
        let mut literal = String::new();

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() || c == '_' {
                literal.push(c);
                self.advance();
            } else {
                break;
            }
        }

        if self.peek() == Some('.') && self.peek_ahead(1).map_or(false, |c| c.is_ascii_digit()) {
            literal.push('.');
            self.advance();

            while let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    literal.push(c);
                    self.advance();
                } else {
                    break;
                }
            }
        }

        Ok(literal)
    }

    fn handle_x_token(&mut self, coefficient: f32) -> Result<Token, String> {
        let after_power = self
            .previous_token_type
            .as_ref()
            .map_or(false, |t| *t == TokenType::Power);

        if coefficient != 1.0 {
            // Queue multiple tokens
            if after_power {
                self.pending_tokens.push_back(Token {
                    token_type: TokenType::OpenParen,
                    numeric_value_1: 0.0,
                    numeric_value_2: 0.0,
                });
            }

            self.pending_tokens.push_back(Token {
                token_type: TokenType::Number,
                numeric_value_1: coefficient,
                numeric_value_2: 0.0,
            });

            self.pending_tokens.push_back(Token {
                token_type: TokenType::Star,
                numeric_value_1: 0.0,
                numeric_value_2: 0.0,
            });

            self.pending_tokens.push_back(Token {
                token_type: TokenType::X,
                numeric_value_1: 1.0,
                numeric_value_2: 1.0,
            });

            if after_power {
                self.pending_tokens.push_back(Token {
                    token_type: TokenType::CloseParen,
                    numeric_value_1: 0.0,
                    numeric_value_2: 0.0,
                });
            }

            // Return the first token
            let first_token = self.pending_tokens.pop_front().unwrap();
            self.previous_token_type = Some(first_token.token_type);
            Ok(first_token)
        } else {
            Ok(self.make_token_with_values(TokenType::X, 1.0, 1.0))
        }
    }

    fn scan_token(&mut self) -> Result<Option<Token>, String> {
        use TokenType::*;

        // Skip whitespace
        while let Some(c) = self.peek() {
            if !matches!(c, ' ' | '\r' | '\t') {
                break;
            }
            self.advance();
        }

        // Check if we're done
        if self.peek().is_none() {
            if !self.finished {
                self.finished = true;
                return Ok(Some(self.make_token(End)));
            }
            return Ok(None);
        }

        self.start_position = self.position;
        let c = self.advance().unwrap();

        let token = match c {
            'y' => self.make_token(Y),
            '=' => self.make_token(Equal),
            ',' => self.make_token(Comma),
            'π' => self.make_token(_Pi),
            'e' => self.make_token(_E),
            '*' => self.make_token(Star),
            '/' => self.make_token(Slash),
            '+' => self.make_token(Plus),
            '!' => self.make_token(Factorial),
            '%' => {
                if self.peek() == Some('%') {
                    self.advance();
                    self.make_token(Modulo)
                } else {
                    self.make_token(Percent)
                }
            }
            '-' => {
                if self.previous_match(&[_E, _Pi, Number, CloseParen, X, Factorial]) {
                    self.make_token(Minus)
                } else if self.peek() == Some('e') {
                    self.advance();
                    self.make_token(NegE)
                } else if self.peek() == Some('π') {
                    self.advance();
                    self.make_token(NegPi)
                } else if self.peek().map_or(false, |c| c.is_ascii_digit()) {
                    let literal = self.scan_digit()?;
                    if self.peek() != Some('x') {
                        let val: f32 = literal.parse().map_err(|_| format!("Invalid number: {}", literal))?;
                        self.make_token_with_values(Number, -val, 0.0)  // Negate!
                    } else {
                        self.advance(); // consume 'x'
                        let coef: f32 = literal.parse().map_err(|_| format!("Invalid number: {}", literal))?;
                        return Ok(Some(self.handle_x_token(-coef)?));  // Negate!
                    }
                } else if self.peek() == Some('x') {
                    self.advance();
                    return Ok(Some(self.handle_x_token(-1.0)?));
                } else if matches!(self.peek(), Some('(') | Some('-')) || self.peek().map_or(false, |c| c.is_alphabetic()) {
                    // -(5) or -sqrt(4) or --2
                    // Queue Star token for next iteration
                    self.pending_tokens.push_back(Token {
                        token_type: Star,
                        numeric_value_1: 0.0,
                        numeric_value_2: 0.0,
                    });
                    self.make_token_with_values(Number, -1.0, 0.0)
                } else {
                    return Err(format!("Invalid minus sign at position {}", self.position));
                }
            }
            '(' => self.make_token(OpenParen),
            ')' => self.make_token(CloseParen),
            '^' => self.make_token(Power),
            'x' => return Ok(Some(self.handle_x_token(1.0)?)),
            _ if c.is_ascii_digit() => {
                // Put the digit back conceptually by starting from start_position
                self.position = self.start_position;
                self.chars = self.eq[self.position..].chars().peekable();

                let literal = self.scan_digit()?;
                if self.peek() != Some('x') {
                    let val: f32 = literal.parse().map_err(|_| format!("Invalid number: {}", literal))?;
                    self.make_token_with_values(Number, val, 0.0)
                } else {
                    self.advance(); // consume 'x'
                    let coef: f32 = literal.parse().map_err(|_| format!("Invalid number: {}", literal))?;
                    return Ok(Some(self.handle_x_token(coef)?));
                }
            }
            _ if c.is_alphabetic() => {
                // Scan function name
                self.position = self.start_position;
                self.chars = self.eq[self.position..].chars().peekable();

                let mut name = String::new();
                while let Some(ch) = self.peek() {
                    if ch.is_alphabetic() {
                        name.push(ch);
                        self.advance();
                    } else {
                        break;
                    }
                }

                // Handle log base
                if self.peek() == Some('_') {
                    if name != "log" {
                        return Err(format!("Invalid input at character {}", self.position));
                    }
                    self.advance(); // consume '_'

                    if !self.peek().map_or(false, |c| c.is_ascii_digit()) {
                        return Err("Invalid use of log".to_string());
                    }

                    let base_literal = self.scan_digit()?;
                    let base: f32 = base_literal.parse().map_err(|_| "Invalid log base".to_string())?;

                    if self.peek() != Some('(') {
                        return Err(format!("Invalid input at character {}", self.start_position));
                    }
                    self.advance(); // consume '('

                    return Ok(Some(self.make_token_with_values(Log, base, 0.0)));
                }

                if self.peek() != Some('(') {
                    return Err(format!("Invalid input at character {}", self.start_position));
                }
                self.advance(); // consume '('

                let token_type = match name.as_str() {
                    "sin" => Sin,
                    "cos" => Cos,
                    "tan" => Tan,
                    "asin" => Asin,
                    "acos" => Acos,
                    "atan" => Atan,
                    "max" => Max,
                    "abs" => Abs,
                    "sqrt" => Sqrt,
                    "min" => Min,
                    "ln" => Ln,
                    "avg" => Avg,
                    "med" => Med,
                    "mode" => Mode,
                    "ch" => Choice,
                    _ => return Err(format!("Invalid function name {}", name)),
                };

                self.make_token(token_type)
            }
            _ => return Err(format!("Invalid input at character {}", self.position)),
        };

        Ok(Some(token))
    }
}

impl<'a> Iterator for StreamingTokenizer<'a> {
    type Item = Result<Token, String>;

    fn next(&mut self) -> Option<Self::Item> {
        // First check if we have pending tokens from multi-token operations
        if let Some(token) = self.pending_tokens.pop_front() {
            self.previous_token_type = Some(token.token_type);
            return Some(Ok(token));
        }

        // Otherwise scan the next token
        match self.scan_token() {
            Ok(Some(token)) => Some(Ok(token)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}
