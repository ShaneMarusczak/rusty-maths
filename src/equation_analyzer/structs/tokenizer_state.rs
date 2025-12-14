use crate::equation_analyzer::structs::token::{Token, TokenType};

pub(crate) struct TokenizerState<'a> {
    pub(crate) tokens: Vec<Token>,
    pub(crate) start: usize,
    pub(crate) current: usize,
    pub(crate) eq: &'a str,
}

pub(crate) trait Tokenizer {
    fn advance(&mut self) -> Result<char, String>;

    fn peek(&self) -> Result<char, String>;

    fn peek_n(&self, distance: usize) -> Result<char, String>;

    fn previous_match(&self, types: &[TokenType]) -> bool;

    fn at_end(&self) -> bool;

    fn add_token_n(&mut self, token_type: TokenType, numeric_value_1: f32, numeric_value_2: f32);

    fn add_token(&mut self, token_type: TokenType);

    fn take_x(&mut self, coefficient: String) -> Result<(), String>;

    fn digit(&mut self) -> Result<(), String>;
}

impl Tokenizer for TokenizerState<'_> {
    fn advance(&mut self) -> Result<char, String> {
        let c = self.eq.chars().nth(self.current);
        self.current += 1;
        if let Some(c) = c {
            Ok(c)
        } else {
            Err(format!("Invalid Input at {}", self.current))
        }
    }

    fn peek(&self) -> Result<char, String> {
        self.peek_n(0)
    }

    fn peek_n(&self, distance: usize) -> Result<char, String> {
        if self.at_end() {
            return Ok('\0');
        };
        let c = self.eq.chars().nth(self.current + distance);
        if let Some(c) = c {
            Ok(c)
        } else {
            Err(format!("Invalid Input at {}", self.current + distance))
        }
    }

    fn previous_match(&self, types: &[TokenType]) -> bool {
        // If tokens is empty, no previous token matches
        let Some(prev) = self.tokens.last() else {
            return false;
        };

        for tt in types {
            if prev.token_type == *tt {
                return true;
            }
        }
        false
    }

    fn at_end(&self) -> bool {
        self.current >= self.eq.chars().count()
    }

    fn add_token_n(&mut self, token_type: TokenType, numeric_value_1: f32, numeric_value_2: f32) {
        let token = Token {
            token_type,
            numeric_value_1,
            numeric_value_2,
        };
        self.tokens.push(token);
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_n(token_type, 0.0, 0.0);
    }

    fn take_x(&mut self, coefficient: String) -> Result<(), String> {
        let coef: f32 = coefficient.parse()
            .map_err(|_| format!("Invalid coefficient: {}", coefficient))?;
        // Check if previous token was Power - if so, we need to wrap this in parens
        let after_power = self.tokens.last().is_some_and(|t| t.token_type == TokenType::Power);

        if coef != 1.0 {
            if after_power {
                self.add_token(TokenType::OpenParen);
            }
            self.add_token_n(TokenType::Number, coef, 0.0);
            self.add_token(TokenType::Star);
        }
        self.add_token_n(TokenType::X, 1.0, 1.0);
        if after_power && coef != 1.0 {
            self.add_token(TokenType::CloseParen);
        }
        Ok(())
    }

    fn digit(&mut self) -> Result<(), String> {
        while self.peek()?.is_ascii_digit() || self.peek()? == '_' {
            self.advance()?;
        }
        if self.peek()? == '.' && self.peek_n(1)?.is_ascii_digit() {
            //consume the .
            self.advance()?;
            while self.peek()?.is_ascii_digit() {
                self.advance()?;
            }
        }
        Ok(())
    }
}
