use crate::equation_analyzer::catalog::{self, Symbol, SymbolKind};
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

    fn previous_match(&self, types: &[TokenType]) -> bool {
        self.previous_token_type
            .as_ref()
            .is_some_and(|prev| types.contains(prev))
    }

    fn make_token(&mut self, token_type: TokenType) -> Token {
        self.make_token_with_values(token_type, 0.0, 0.0)
    }

    fn make_token_with_values(&mut self, token_type: TokenType, val1: f32, val2: f32) -> Token {
        let token = Token {
            token_type,
            numeric_value_1: val1,
            numeric_value_2: val2,
            symbol: None,
        };
        self.previous_token_type = Some(token_type);
        token
    }

    fn make_token_with_symbol(&mut self, token_type: TokenType, symbol: &'static Symbol) -> Token {
        self.previous_token_type = Some(token_type);
        Token {
            token_type,
            numeric_value_1: 0.0,
            numeric_value_2: 0.0,
            symbol: Some(symbol),
        }
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

        if self.peek() == Some('.') && self.peek_ahead(1).is_some_and(|c| c.is_ascii_digit()) {
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
        // Check if we need to wrap in parentheses (after operators with precedence >= 3)
        // We wrap after: Power(4), Star(3), Slash(3), Modulo(3), Percent(3)
        // We don't wrap after Plus(2) or Minus(2) because * has higher precedence
        let needs_parens = self.previous_token_type.as_ref().is_some_and(|t| {
            matches!(
                t,
                TokenType::Power
                    | TokenType::Star
                    | TokenType::Slash
                    | TokenType::Modulo
                    | TokenType::Percent
            )
        });

        if coefficient != 1.0 {
            // Queue multiple tokens
            if needs_parens {
                self.pending_tokens.push_back(Token {
                    token_type: TokenType::OpenParen,
                    numeric_value_1: 0.0,
                    numeric_value_2: 0.0,
                    symbol: None,
                });
            }

            self.pending_tokens.push_back(Token {
                token_type: TokenType::Number,
                numeric_value_1: coefficient,
                numeric_value_2: 0.0,
                symbol: None,
            });

            self.pending_tokens.push_back(Token {
                token_type: TokenType::Star,
                numeric_value_1: 0.0,
                numeric_value_2: 0.0,
                symbol: None,
            });

            self.pending_tokens.push_back(Token {
                token_type: TokenType::X,
                numeric_value_1: 1.0,
                numeric_value_2: 1.0,
                symbol: None,
            });

            if needs_parens {
                self.pending_tokens.push_back(Token {
                    token_type: TokenType::CloseParen,
                    numeric_value_1: 0.0,
                    numeric_value_2: 0.0,
                    symbol: None,
                });
            }

            // Return the first token (we just pushed at least one token above)
            let first_token = self
                .pending_tokens
                .pop_front()
                .ok_or_else(|| String::from("Internal error: expected token in pending queue"))?;
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
        let c = self
            .advance()
            .ok_or_else(|| String::from("Unexpected end of input"))?;

        let token = match c {
            'y' => self.make_token(Y),
            '=' => self.make_token(Equal),
            ',' => self.make_token(Comma),
            'π' => {
                let sym = catalog::find("π")
                    .ok_or_else(|| String::from("Internal: catalog missing π"))?;
                self.make_token_with_symbol(TokenType::Constant, sym)
            }
            'e' => {
                let sym = catalog::find("e")
                    .ok_or_else(|| String::from("Internal: catalog missing e"))?;
                self.make_token_with_symbol(TokenType::Constant, sym)
            }
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
                // Check if this is binary minus (subtraction) or unary minus (negation)
                // Percent is in the operand list because it's postfix: `50% - 3`.
                if self.previous_match(&[Constant, Number, CloseParen, X, Factorial, Percent]) {
                    // Previous token was an operand, so this is binary subtraction
                    self.make_token(Minus)
                } else {
                    // Previous token was an operator, '(', or start of input - this is unary negation
                    self.make_token(UnaryMinus)
                }
            }
            '|' => {
                // Accept either `|` or `|>` as the pipe operator.
                if self.peek() == Some('>') {
                    self.advance();
                }
                self.make_token(Pipe)
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
                    let val: f32 = literal
                        .parse()
                        .map_err(|_| format!("Invalid number: {}", literal))?;
                    self.make_token_with_values(Number, val, 0.0)
                } else {
                    self.advance(); // consume 'x'
                    let coef: f32 = literal
                        .parse()
                        .map_err(|_| format!("Invalid number: {}", literal))?;
                    return Ok(Some(self.handle_x_token(coef)?));
                }
            }
            _ if c.is_alphabetic() => {
                // Scan function name
                self.position = self.start_position;
                self.chars = self.eq[self.position..].chars().peekable();

                let mut name = String::new();
                while let Some(ch) = self.peek() {
                    if ch.is_alphabetic() || (!name.is_empty() && ch.is_ascii_digit()) {
                        name.push(ch);
                        self.advance();
                    } else {
                        break;
                    }
                }

                // Pipe target: name must be a unary function, no parens follow.
                if matches!(self.previous_token_type, Some(Pipe)) {
                    let sym = catalog::find(&name)
                        .filter(|s| s.kind.is_unary())
                        .ok_or_else(|| format!(
                            "'{}' cannot be used after '|>'; only unary functions are allowed",
                            name
                        ))?;

                    if self.peek() == Some('(') {
                        return Err(format!(
                            "Function '{}' after '|>' must not be called with parentheses; the piped value is its argument",
                            name
                        ));
                    }

                    return Ok(Some(self.make_token_with_symbol(TokenType::Call, sym)));
                }

                // Handle log base
                if self.peek() == Some('_') {
                    if name != "log" {
                        return Err(format!("Invalid input at character {}", self.position));
                    }
                    self.advance(); // consume '_'

                    if !self.peek().is_some_and(|c| c.is_ascii_digit()) {
                        return Err("Invalid use of log".to_string());
                    }

                    let base_literal = self.scan_digit()?;
                    let base: f32 = base_literal
                        .parse()
                        .map_err(|_| "Invalid log base".to_string())?;

                    if self.peek() != Some('(') {
                        return Err(format!(
                            "Invalid input at character {}",
                            self.start_position
                        ));
                    }
                    self.advance(); // consume '('

                    return Ok(Some(self.make_token_with_values(Log, base, 0.0)));
                }

                // Bare constant written out by name (e.g. `pi` for π). The
                // single-char constants (π, e) are matched earlier at the
                // character level; this path covers multi-char aliases.
                if let Some(sym) = catalog::find(&name)
                    .filter(|s| matches!(s.kind, SymbolKind::Constant(_)))
                {
                    return Ok(Some(self.make_token_with_symbol(TokenType::Constant, sym)));
                }

                // Word operators (`17 mod 5`). The catalog documents them;
                // each one maps to its structural TokenType here.
                if catalog::find(&name)
                    .is_some_and(|s| matches!(s.kind, SymbolKind::Operator { .. }))
                {
                    let token_type = match name.as_str() {
                        "mod" => Modulo,
                        _ => {
                            return Err(format!(
                                "Operator '{}' cannot be written as a word here",
                                name
                            ));
                        }
                    };
                    return Ok(Some(self.make_token(token_type)));
                }

                if self.peek() != Some('(') {
                    return Err(format!(
                        "Invalid input at character {}",
                        self.start_position
                    ));
                }
                self.advance(); // consume '('

                let sym = catalog::find(&name)
                    .filter(|s| s.kind.is_unary() || s.kind.is_variadic())
                    .ok_or_else(|| format!("Invalid function name {}", name))?;

                self.make_token_with_symbol(TokenType::Call, sym)
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
