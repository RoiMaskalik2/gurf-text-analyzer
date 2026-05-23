//! Errors that can occur in this crate, grouped by the module they came from.
use std::io;

/// Represents an error that can occur while using the text analyzer crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    //------------user_input-------------------
    /// User provided an empty input.
    #[error("{self:?}")]
    EmptyString,

    /// Error occurred during input reading.
    #[error("{self:?}")]
    Io(#[from] io::Error),

    // ------------text_analyzer---------------
    /// Access a word frequencies of a word that did not exist in the text analyzer
    #[error("{self:?}")]
    NonExistingWord,

    /// The provided word is invalid - meaning it is empty or has spaces.
    #[error("{self:?}")]
    InvalidWord,

    /// The provided text is invalid - meaning it is empty or contains only spaces
    #[error("{self:?}")]
    InvalidText,
}
