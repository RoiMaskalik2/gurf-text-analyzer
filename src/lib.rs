//! Text Analyzer
//!
//! Exports the building blocks for analyzing the frequencies of words inside texts.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod err;
pub mod text_analyzer;
pub mod user_input;

pub use err::{Error, Result};
pub use text_analyzer::TextAnalyzer;
