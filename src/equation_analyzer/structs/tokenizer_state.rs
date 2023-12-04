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
        if self.tokens.is_empty() {
            return false;
        }
        let prev = self.tokens.last().unwrap();

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
