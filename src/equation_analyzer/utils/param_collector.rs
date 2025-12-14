/// Parameter collector for variadic functions in RPN evaluation
///
/// This module handles the collection and evaluation of parameters for variadic
/// functions like avg(), min(), max(), mode(), median(), and choice().
///
/// The collector uses a state machine:
/// - When a variadic function token is encountered, it enters collecting mode
/// - It accumulates Number and X tokens into a parameter vector
/// - When an End* synthetic token is encountered, it computes the result and exits
///
/// **Limitation:** Nested variadic functions are not supported. For example,
/// `avg(1, min(2, 3), 4)` will produce an error. Only simple parameters are allowed.
use crate::{
    equation_analyzer::structs::token::{Token, TokenType},
    utilities::factorial,
};
use std::collections::HashMap;

/// Result of processing a token during parameter collection
pub enum CollectionResult {
    /// Not currently collecting parameters (caller should process token normally)
    NotCollecting,
    /// Token was consumed as a parameter, continue to next token
    Continue,
    /// Collection finished with a result to push to the stack
    Finished(Result<f32, String>),
}

/// Handles parameter collection and evaluation for variadic functions
pub struct ParamCollector {
    /// Whether we're currently collecting parameters
    collecting: bool,
    /// Accumulated parameters
    params: Vec<f32>,
}

impl ParamCollector {
    /// Creates a new parameter collector in non-collecting state
    pub fn new() -> Self {
        Self {
            collecting: false,
            params: Vec::new(),
        }
    }

    /// Returns true if currently collecting parameters
    #[cfg(test)]
    pub fn is_collecting(&self) -> bool {
        self.collecting
    }

    /// Enters parameter collection mode
    pub fn start_collecting(&mut self) {
        self.collecting = true;
        self.params.clear();
    }

    /// Processes a token during parameter collection
    ///
    /// Returns:
    /// - `CollectionResult::NotCollecting` if not in collection mode
    /// - `CollectionResult::Continue` if token was consumed (Number/X added to params)
    /// - `CollectionResult::Finished(Ok(value))` if collection completed
    /// - `CollectionResult::Finished(Err(msg))` if an error occurred
    pub fn process_token(&mut self, token: &Token, x: f32) -> CollectionResult {
        if !self.collecting {
            return CollectionResult::NotCollecting;
        }

        match token.token_type {
            TokenType::Number => {
                self.params.push(token.numeric_value_1);
                CollectionResult::Continue
            }
            TokenType::X => {
                self.params
                    .push(token.numeric_value_1 * x.powf(token.numeric_value_2));
                CollectionResult::Continue
            }
            TokenType::EndAvg => CollectionResult::Finished(self.finish_avg()),
            TokenType::EndMin => CollectionResult::Finished(self.finish_min()),
            TokenType::EndMax => CollectionResult::Finished(self.finish_max()),
            TokenType::EndMode => CollectionResult::Finished(self.finish_mode()),
            TokenType::EndMed => CollectionResult::Finished(self.finish_median()),
            TokenType::EndChoice => CollectionResult::Finished(self.finish_choice()),
            _ => CollectionResult::Finished(Err(format!(
                "Invalid token in parameter collection: {:?}. Only Number and X tokens are allowed as parameters. Nested variadic functions are not supported.",
                token.token_type
            ))),
        }
    }

    /// Resets the collector to non-collecting state
    fn reset(&mut self) {
        self.collecting = false;
        self.params.clear();
    }

    /// Validates parameter count is within range
    fn validate_param_count(&self, min: usize, max: usize) -> Result<(), String> {
        let count = self.params.len();
        if count < min {
            return Err(format!(
                "Expected at least {} parameter{}, got {}",
                min,
                if min == 1 { "" } else { "s" },
                count
            ));
        }
        if count > max && max != usize::MAX {
            return Err(format!("Expected at most {} parameters, got {}", max, count));
        }
        Ok(())
    }

    /// Validates all parameters are integers (for functions like choice)
    fn validate_all_integers(&self) -> Result<(), String> {
        for (i, &param) in self.params.iter().enumerate() {
            if param % 1.0 != 0.0 {
                return Err(format!(
                    "Parameter {} must be an integer, got {}",
                    i + 1,
                    param
                ));
            }
            if param < 0.0 {
                return Err(format!(
                    "Parameter {} must be non-negative, got {}",
                    i + 1,
                    param
                ));
            }
        }
        Ok(())
    }

    /// Computes average of collected parameters
    fn finish_avg(&mut self) -> Result<f32, String> {
        self.validate_param_count(1, usize::MAX)?;
        let result = self.params.iter().sum::<f32>() / self.params.len() as f32;
        self.reset();
        Ok(result)
    }

    /// Computes minimum of collected parameters
    fn finish_min(&mut self) -> Result<f32, String> {
        self.validate_param_count(1, usize::MAX)?;
        let result = self.params.iter().copied().fold(f32::MAX, f32::min);
        self.reset();
        Ok(result)
    }

    /// Computes maximum of collected parameters
    fn finish_max(&mut self) -> Result<f32, String> {
        self.validate_param_count(1, usize::MAX)?;
        let result = self.params.iter().copied().fold(f32::MIN, f32::max);
        self.reset();
        Ok(result)
    }

    /// Computes mode of collected parameters
    ///
    /// Returns:
    /// - NaN for uniform distributions (all values appear with same frequency)
    /// - Average of all modes for multimodal distributions
    /// - The mode for unimodal distributions
    fn finish_mode(&mut self) -> Result<f32, String> {
        self.validate_param_count(1, usize::MAX)?;

        // Build frequency map
        let mut seen: HashMap<u32, usize> = HashMap::new();
        for param in self.params.iter() {
            let bits = param.to_bits();
            let count = seen.entry(bits).or_insert(0);
            *count += 1;
        }

        // Safety: validate_param_count ensures at least 1 param, so seen is not empty
        let max_count = *seen.values().max()
            .ok_or_else(|| String::from("mode requires at least one parameter"))?;

        let result = if max_count == 1 {
            // Uniform distribution: all values appear with same frequency
            f32::NAN
        } else {
            // Collect all values with max frequency (handles multimodal)
            let modes: Vec<f32> = seen
                .iter()
                .filter(|(_, &count)| count == max_count)
                .map(|(&bits, _)| f32::from_bits(bits))
                .collect();

            // Return average of all modes
            modes.iter().sum::<f32>() / modes.len() as f32
        };

        self.reset();
        Ok(result)
    }

    /// Computes median of collected parameters
    fn finish_median(&mut self) -> Result<f32, String> {
        self.validate_param_count(1, usize::MAX)?;

        self.params
            .sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let len = self.params.len();
        let result = if len.is_multiple_of(2) {
            let mid = len / 2;
            (self.params[mid - 1] + self.params[mid]) / 2.0
        } else {
            self.params[len / 2]
        };

        self.reset();
        Ok(result)
    }

    /// Computes binomial coefficient (n choose k)
    fn finish_choice(&mut self) -> Result<f32, String> {
        self.validate_param_count(2, 2)?;
        self.validate_all_integers()?;

        let n = self.params[0] as isize;
        let k = self.params[1] as isize;

        if k > n {
            let result = 0.0;
            self.reset();
            return Ok(result);
        }

        let result = (factorial(n) / (factorial(k) * factorial(n - k))) as f32;
        self.reset();
        Ok(result)
    }
}

impl Default for ParamCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_number(val: f32) -> Token {
        Token {
            token_type: TokenType::Number,
            numeric_value_1: val,
            numeric_value_2: 0.0,
        }
    }

    fn make_end_token(tt: TokenType) -> Token {
        Token {
            token_type: tt,
            numeric_value_1: 0.0,
            numeric_value_2: 0.0,
        }
    }

    #[test]
    fn test_avg_basic() {
        let mut collector = ParamCollector::new();
        collector.start_collecting();

        assert!(matches!(
            collector.process_token(&make_number(1.0), 0.0),
            CollectionResult::Continue
        ));
        assert!(matches!(
            collector.process_token(&make_number(2.0), 0.0),
            CollectionResult::Continue
        ));
        assert!(matches!(
            collector.process_token(&make_number(3.0), 0.0),
            CollectionResult::Continue
        ));

        match collector.process_token(&make_end_token(TokenType::EndAvg), 0.0) {
            CollectionResult::Finished(Ok(result)) => {
                assert_eq!(result, 2.0);
                assert!(!collector.is_collecting());
            }
            _ => panic!("Expected Finished(Ok(...))"),
        }
    }

    #[test]
    fn test_min_basic() {
        let mut collector = ParamCollector::new();
        collector.start_collecting();

        collector.process_token(&make_number(3.0), 0.0);
        collector.process_token(&make_number(1.0), 0.0);
        collector.process_token(&make_number(2.0), 0.0);

        match collector.process_token(&make_end_token(TokenType::EndMin), 0.0) {
            CollectionResult::Finished(Ok(result)) => assert_eq!(result, 1.0),
            _ => panic!("Expected Finished(Ok(...))"),
        }
    }

    #[test]
    fn test_max_basic() {
        let mut collector = ParamCollector::new();
        collector.start_collecting();

        collector.process_token(&make_number(1.0), 0.0);
        collector.process_token(&make_number(5.0), 0.0);
        collector.process_token(&make_number(3.0), 0.0);

        match collector.process_token(&make_end_token(TokenType::EndMax), 0.0) {
            CollectionResult::Finished(Ok(result)) => assert_eq!(result, 5.0),
            _ => panic!("Expected Finished(Ok(...))"),
        }
    }

    #[test]
    fn test_mode_uniform() {
        let mut collector = ParamCollector::new();
        collector.start_collecting();

        collector.process_token(&make_number(1.0), 0.0);
        collector.process_token(&make_number(2.0), 0.0);
        collector.process_token(&make_number(3.0), 0.0);

        match collector.process_token(&make_end_token(TokenType::EndMode), 0.0) {
            CollectionResult::Finished(Ok(result)) => assert!(result.is_nan()),
            _ => panic!("Expected Finished(Ok(...))"),
        }
    }

    #[test]
    fn test_mode_multimodal() {
        let mut collector = ParamCollector::new();
        collector.start_collecting();

        collector.process_token(&make_number(1.0), 0.0);
        collector.process_token(&make_number(1.0), 0.0);
        collector.process_token(&make_number(3.0), 0.0);
        collector.process_token(&make_number(3.0), 0.0);

        match collector.process_token(&make_end_token(TokenType::EndMode), 0.0) {
            CollectionResult::Finished(Ok(result)) => assert_eq!(result, 2.0),
            _ => panic!("Expected Finished(Ok(...))"),
        }
    }

    #[test]
    fn test_median_odd() {
        let mut collector = ParamCollector::new();
        collector.start_collecting();

        collector.process_token(&make_number(3.0), 0.0);
        collector.process_token(&make_number(1.0), 0.0);
        collector.process_token(&make_number(2.0), 0.0);

        match collector.process_token(&make_end_token(TokenType::EndMed), 0.0) {
            CollectionResult::Finished(Ok(result)) => assert_eq!(result, 2.0),
            _ => panic!("Expected Finished(Ok(...))"),
        }
    }

    #[test]
    fn test_median_even() {
        let mut collector = ParamCollector::new();
        collector.start_collecting();

        collector.process_token(&make_number(1.0), 0.0);
        collector.process_token(&make_number(2.0), 0.0);
        collector.process_token(&make_number(3.0), 0.0);
        collector.process_token(&make_number(4.0), 0.0);

        match collector.process_token(&make_end_token(TokenType::EndMed), 0.0) {
            CollectionResult::Finished(Ok(result)) => assert_eq!(result, 2.5),
            _ => panic!("Expected Finished(Ok(...))"),
        }
    }

    #[test]
    fn test_choice_basic() {
        let mut collector = ParamCollector::new();
        collector.start_collecting();

        collector.process_token(&make_number(5.0), 0.0);
        collector.process_token(&make_number(2.0), 0.0);

        match collector.process_token(&make_end_token(TokenType::EndChoice), 0.0) {
            CollectionResult::Finished(Ok(result)) => assert_eq!(result, 10.0),
            _ => panic!("Expected Finished(Ok(...))"),
        }
    }

    #[test]
    fn test_choice_validation_float() {
        let mut collector = ParamCollector::new();
        collector.start_collecting();

        collector.process_token(&make_number(5.5), 0.0);
        collector.process_token(&make_number(2.0), 0.0);

        match collector.process_token(&make_end_token(TokenType::EndChoice), 0.0) {
            CollectionResult::Finished(Err(_)) => {} // Expected
            _ => panic!("Expected Finished(Err(...))"),
        }
    }

    #[test]
    fn test_min_single_param() {
        let mut collector = ParamCollector::new();
        collector.start_collecting();

        collector.process_token(&make_number(42.0), 0.0);

        match collector.process_token(&make_end_token(TokenType::EndMin), 0.0) {
            CollectionResult::Finished(Ok(result)) => assert_eq!(result, 42.0),
            _ => panic!("Expected Finished(Ok(42.0))"),
        }
    }

    #[test]
    fn test_avg_validation_empty() {
        let mut collector = ParamCollector::new();
        collector.start_collecting();

        match collector.process_token(&make_end_token(TokenType::EndAvg), 0.0) {
            CollectionResult::Finished(Err(_)) => {} // Expected
            _ => panic!("Expected Finished(Err(...))"),
        }
    }

    #[test]
    fn test_invalid_token_error() {
        let mut collector = ParamCollector::new();
        collector.start_collecting();

        let invalid_token = Token {
            token_type: TokenType::Plus,
            numeric_value_1: 0.0,
            numeric_value_2: 0.0,
        };

        match collector.process_token(&invalid_token, 0.0) {
            CollectionResult::Finished(Err(msg)) => {
                assert!(msg.contains("Invalid token in parameter collection"));
            }
            _ => panic!("Expected Finished(Err(...))"),
        }
    }
}
