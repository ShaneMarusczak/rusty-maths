use crate::equation_analyzer::structs::operands::{get_operator, Assoc, Operand};
use crate::equation_analyzer::structs::token::{ParamToken, Token, TokenType};
use crate::equation_analyzer::utils::make_synthetic_token;
use std::collections::VecDeque;

/// A fully streaming parser that implements Iterator, yielding RPN tokens on demand.
///
/// This is the core streaming parser shared by full_pipeline implementations.
/// Unlike the buffered Shunting Yard parser, this maintains only a minimal operator stack
/// (partial buffer) and yields tokens as soon as they can be output in RPN order.
///
/// The evaluator can pull tokens one at a time without waiting for the entire expression to be parsed.
///
/// # Architecture
/// - Input: Iterator of tokens (lazy)
/// - Output: Iterator of RPN tokens (lazy)
/// - State: Only algorithm-required buffers (operator stack, output queue)
pub(crate) struct FullyStreamingParser<I>
where
    I: Iterator<Item = Result<Token, String>>,
{
    tokens: I,
    operator_stack: Vec<Operand>,
    output_queue: VecDeque<Token>,
    paren_depth: i32,
    param_token: ParamToken,
    finished: bool,
    found_end: bool,
}

impl<I> FullyStreamingParser<I>
where
    I: Iterator<Item = Result<Token, String>>,
{
    pub(crate) fn new(tokens: I) -> Self {
        Self {
            tokens,
            operator_stack: Vec::new(),
            output_queue: VecDeque::new(),
            paren_depth: 0,
            param_token: ParamToken::None,
            finished: false,
            found_end: false,
        }
    }

    fn process_token(&mut self, token: Token) -> Result<(), String> {
        // Handle variadic function parameter collection
        if self.param_token != ParamToken::None {
            if matches!(token.token_type, TokenType::Number | TokenType::X) {
                self.output_queue.push_back(token);
                return Ok(());
            } else if token.token_type == TokenType::Comma {
                return Ok(());
            } else if token.token_type == TokenType::CloseParen {
                self.output_queue.push_back(make_synthetic_token(self.param_token.to_end_token_type()));
                self.param_token = ParamToken::None;
                return Ok(());
            }

            return Err("Params can only be numbers".to_string());
        }

        match token.token_type {
            // Skip equation markers and commas
            TokenType::Y | TokenType::Equal | TokenType::Comma => Ok(()),

            // Constants go directly to output queue
            TokenType::_Pi | TokenType::_E | TokenType::NegPi | TokenType::NegE => {
                self.output_queue.push_back(token);
                Ok(())
            }

            // Variadic functions - start parameter collection
            TokenType::Avg => {
                self.output_queue.push_back(token);
                self.param_token = ParamToken::Avg;
                Ok(())
            }

            TokenType::Choice => {
                self.output_queue.push_back(token);
                self.param_token = ParamToken::Choice;
                Ok(())
            }

            TokenType::Min => {
                self.output_queue.push_back(token);
                self.param_token = ParamToken::Min;
                Ok(())
            }

            TokenType::Max => {
                self.output_queue.push_back(token);
                self.param_token = ParamToken::Max;
                Ok(())
            }

            TokenType::Med => {
                self.output_queue.push_back(token);
                self.param_token = ParamToken::Med;
                Ok(())
            }

            TokenType::Mode => {
                self.output_queue.push_back(token);
                self.param_token = ParamToken::Mode;
                Ok(())
            }

            // Functions and opening parenthesis go on operator stack
            TokenType::Sin
            | TokenType::Cos
            | TokenType::Tan
            | TokenType::Asin
            | TokenType::Acos
            | TokenType::Atan
            | TokenType::Abs
            | TokenType::Sqrt
            | TokenType::Ln
            | TokenType::Log
            | TokenType::OpenParen => {
                self.paren_depth += 1;
                self.operator_stack.push(get_operator(token));
                Ok(())
            }

            // Closing parenthesis: pop operators until matching open paren
            TokenType::CloseParen => {
                self.paren_depth -= 1;

                if self.paren_depth < 0 {
                    return Err("Invalid closing parenthesis".to_string());
                }

                // Pop operators until we find the matching opening parenthesis
                while !self.operator_stack.is_empty() {
                    let last = self
                        .operator_stack
                        .last()
                        .ok_or("Missing operator on stack")?;
                    if last.paren_opener {
                        break;
                    }
                    let op = self.operator_stack.pop().unwrap();
                    self.output_queue.push_back(op.token);
                }

                let last_op = self
                    .operator_stack
                    .last()
                    .ok_or("Mismatched parentheses")?;

                // Remove the opening parenthesis
                if last_op.token.token_type == TokenType::OpenParen {
                    self.operator_stack.pop();
                    return Ok(());
                }

                // If there's a function waiting, add it to output queue
                if !self.operator_stack.is_empty() {
                    let last = self
                        .operator_stack
                        .last()
                        .ok_or("Missing operator on stack")?;
                    if last.is_func {
                        let func = self.operator_stack.pop().unwrap();
                        self.output_queue.push_back(func.token);
                    }
                }
                Ok(())
            }

            // Operators: apply precedence and associativity rules
            TokenType::Star
            | TokenType::Slash
            | TokenType::Plus
            | TokenType::Minus
            | TokenType::Power
            | TokenType::Modulo
            | TokenType::Percent
            | TokenType::Factorial => {
                let o_1 = get_operator(token);

                // Pop higher precedence operators from stack to output queue
                while !self.operator_stack.is_empty() {
                    let last = self
                        .operator_stack
                        .last()
                        .ok_or("Missing operator on stack")?;

                    if last.paren_opener {
                        break;
                    }

                    // Precedence and associativity rules
                    let should_pop = last.prec > o_1.prec
                        || (last.prec == o_1.prec && o_1.assoc == Assoc::Left);

                    if !should_pop {
                        break;
                    }

                    let o_2_new = self.operator_stack.pop().unwrap();
                    self.output_queue.push_back(o_2_new.token);
                }

                self.operator_stack.push(o_1);
                Ok(())
            }

            // Operands go directly to output queue
            TokenType::Number | TokenType::X => {
                self.output_queue.push_back(token);
                Ok(())
            }

            // End token: flush remaining operators and validate
            TokenType::End => {
                self.found_end = true;

                // Flush remaining operators from stack to output queue
                while let Some(op) = self.operator_stack.pop() {
                    if op.token.token_type == TokenType::OpenParen {
                        return Err("Invalid opening parenthesis".to_string());
                    }
                    self.output_queue.push_back(op.token);
                }

                if self.paren_depth != 0 {
                    return Err("Invalid function".to_string());
                }

                Ok(())
            }

            // Synthetic tokens should never appear in input
            _ => Err("Synthetic token should not be here".to_string()),
        }
    }
}

impl<I> Iterator for FullyStreamingParser<I>
where
    I: Iterator<Item = Result<Token, String>>,
{
    type Item = Result<Token, String>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // First check if we have tokens ready to output
            if let Some(token) = self.output_queue.pop_front() {
                return Some(Ok(token));
            }

            // If we've processed all input and queue is empty, we're done
            if self.finished {
                if !self.found_end {
                    return Some(Err("No end token found".to_string()));
                }
                return None;
            }

            // Pull next token from input
            match self.tokens.next() {
                Some(Ok(token)) => {
                    if let Err(e) = self.process_token(token) {
                        self.finished = true;
                        return Some(Err(e));
                    }
                    // Continue loop - we might have queued output tokens
                }
                Some(Err(e)) => {
                    self.finished = true;
                    return Some(Err(e));
                }
                None => {
                    self.finished = true;
                    // Continue loop to check if we have queued tokens or need to emit error
                }
            }
        }
    }
}
