/// Core shared implementations used across all pipeline variants
///
/// This module contains the canonical implementations of tokenizers, parsers, and evaluators
/// that are reused by vec_pipeline, hybrid_pipeline, and full_pipeline to eliminate duplication.

pub mod vec_tokenizer;
pub mod streaming_tokenizer;
