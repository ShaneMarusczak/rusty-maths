/// Core shared implementations used across all pipeline variants
///
/// This module contains the canonical implementations of tokenizers, parsers, and evaluators
/// that are reused by vec_pipeline, hybrid_pipeline, and full_pipeline to eliminate duplication.
///
/// ## Architecture
/// - **vec_tokenizer**: Traditional Vec-based tokenization
/// - **streaming_tokenizer**: Iterator-based lazy tokenization
/// - **parser**: Shunting Yard algorithm (buffered output)
/// - **streaming_parser**: Fully streaming Shunting Yard (lazy output)
/// - **evaluator**: Generic RPN evaluator (works with any iterator)

pub mod vec_tokenizer;
pub mod streaming_tokenizer;
pub mod evaluator;
pub mod parser;
pub mod streaming_parser;
