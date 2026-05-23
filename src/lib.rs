//! Text Analyzer
//!
//! Exports the building blocks for analyzing the frequencies of words inside texts.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod err;
pub mod text_analyzer;
pub mod user_input;

pub use err::Error;
pub use text_analyzer::TextAnalyzer;

/// Type alias for the Result enum so that callers will not need to include the error enum in it.
pub type Result<T> = core::result::Result<T, Error>;
