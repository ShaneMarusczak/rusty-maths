use crate::equation_analyzer::structs::token::{Token, TokenType};
use crate::utilities::is_dig;

pub(crate) struct TokenizerState<'a> {
    pub(crate) tokens: Vec<Token>,
    pub(crate) start: usize,
    pub(crate) current: usize,
    pub(crate) eq: &'a str,
}

pub(crate) trait Tokenizer {
    fn advance(&mut self) -> Result<char, String>;

    fn peek(&self) -> Result<char, String>;

    fn peek_next(&self) -> Result<char, String>;

    fn at_end(&self) -> bool;

    fn add_token(&mut self, token_type: TokenType, literal: &str);

    fn take_x(&mut self, coefficient: String) -> Result<(), String>;

    fn power(&mut self) -> Result<(), String>;

    fn digit(&mut self) -> Result<(), String>;
}

impl Tokenizer for TokenizerState<'_> {
    fn advance(&mut self) -> Result<char, String> {
        let c = self.eq.chars().nth(self.current);
        self.current += 1;
        return if let Some(c) = c {
            Ok(c)
        } else {
            Err(format!("Invalid Input at {}", self.current))
        };
    }

    fn peek(&self) -> Result<char, String> {
        if self.at_end() {
            return Ok('\0');
        };
        let c = self.eq.chars().nth(self.current);
        return if let Some(c) = c {
            Ok(c)
        } else {
            Err(format!("Invalid Input at {}", self.current))
        };
    }

    fn peek_next(&self) -> Result<char, String> {
        if self.at_end() {
            return Ok('\0');
        };
        let c = self.eq.chars().nth(self.current + 1);
        return if let Some(c) = c {
            Ok(c)
        } else {
            Err(format!("Invalid Input at {}", self.current + 1))
        };
    }

    fn at_end(&self) -> bool {
        self.current >= self.eq.chars().count()
    }

    fn add_token(&mut self, token_type: TokenType, literal: &str) {
        let token = Token {
            token_type,
            literal: literal.to_owned(),
        };
        self.tokens.push(token);
    }

    fn take_x(&mut self, coefficient: String) -> Result<(), String> {
        let pow_string = if self.peek()? == '^' {
            let pow_start = self.current;
            self.power()?;
            &self.eq[pow_start..self.current]
        } else {
            "^1"
        };

        let final_literal = coefficient + "x" + pow_string;

        self.add_token(TokenType::X, &final_literal);
        Ok(())
    }

    fn power(&mut self) -> Result<(), String> {
        if is_dig(self.peek_next()?) {
            //consume the ^
            self.advance()?;
            self.digit()?;
            Ok(())
        } else {
            Err("Invalid power".to_string())
        }
    }

    fn digit(&mut self) -> Result<(), String> {
        while is_dig(self.peek()?) {
            self.advance()?;
        }
        if self.peek()? == '.' && is_dig(self.peek_next()?) {
            //consume the .
            self.advance()?;
            while is_dig(self.peek()?) {
                self.advance()?;
            }
        }
        Ok(())
    }
}
