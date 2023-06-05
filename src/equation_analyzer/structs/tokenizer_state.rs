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

    fn previous_match(&self, types: &[TokenType]) -> bool;

    fn at_end(&self) -> bool;

    fn add_token_n(&mut self, token_type: TokenType, numeric_value_1: f32, numeric_value_2: f32);

    fn add_token(&mut self, token_type: TokenType);

    fn take_x(&mut self, coefficient: String) -> Result<(), String>;

    fn power(&mut self) -> Result<(), String>;

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
        if self.at_end() {
            return Ok('\0');
        };
        let c = self.eq.chars().nth(self.current);
        if let Some(c) = c {
            Ok(c)
        } else {
            Err(format!("Invalid Input at {}", self.current))
        }
    }

    fn peek_next(&self) -> Result<char, String> {
        if self.at_end() {
            return Ok('\0');
        };
        let c = self.eq.chars().nth(self.current + 1);
        if let Some(c) = c {
            Ok(c)
        } else {
            Err(format!("Invalid Input at {}", self.current + 1))
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
        self.add_token_n(token_type, 0.0, 0.0)
    }

    fn take_x(&mut self, coefficient: String) -> Result<(), String> {
        let pow_string = if self.peek()? == '^' {
            let pow_start = self.current;
            self.power()?;

            crate::utilities::get_str_section(self.eq, pow_start, self.current)
        } else {
            String::from("^1")
        };

        self.add_token_n(
            TokenType::X,
            coefficient.parse().unwrap(),
            pow_string[1..].parse().unwrap(),
        );
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
        while is_dig(self.peek()?) || self.peek()? == '_' {
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
