// Legacy exports for backwards compatibility
pub mod calculator {
    pub use super::vec_pipeline::calculator::*;
}

// Three pipeline implementations
pub mod vec_pipeline;
pub mod hybrid_pipeline;
pub mod full_pipeline;

// Internal modules
pub(crate) mod structs;
mod tests;

// Legacy pipeline module (kept for internal use by tests)
pub(crate) mod pipeline;

// Legacy calculators (deprecated - use pipeline-specific ones)
pub mod streaming_calculator {
    pub use super::hybrid_pipeline::calculator::*;
}
pub mod fully_streaming_calculator {
    pub use super::full_pipeline::calculator::*;
}
