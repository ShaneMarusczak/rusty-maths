use crate::equation_analyzer::{
    calculator::calculate,
    structs::token::{Token, TokenType},
};

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

    fn take_x(&mut self, coefficient: String) -> Result<(), String> {
        let pow_string = if self.peek()? == '^' {
            let pow_start = self.current;
            self.power()?;

            let pow = crate::utilities::get_str_section(self.eq, pow_start, self.current);
            let p = pow.replace(['^', '(', ')'], "");

            let val = calculate(&p).unwrap();

            format!("{val}")
        } else {
            String::from("1")
        };

        self.add_token_n(
            TokenType::X,
            coefficient.parse().unwrap(),
            pow_string.parse::<f32>().unwrap(),
        );
        Ok(())
    }

    fn power(&mut self) -> Result<(), String> {
        if self.peek_n(1)? == '-' {
            self.advance()?;
        }
        if self.peek_n(1)?.is_ascii_digit() {
            //consume the ^
            self.advance()?;

            self.digit()?;

            Ok(())
        } else if self.peek_n(1)? == '(' {
            self.advance()?;
            self.advance()?;

            self.digit()?;
            if self.peek()? == '/' {
                self.advance()?;
                self.digit()?;
            }
            if self.peek()? != ')' {
                return Err("Invalid power".to_string());
            }
            self.advance()?;
            Ok(())
        } else {
            Err("Invalid power".to_string())
        }
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
