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
    NonExistingWord(String),

    /// The provided word is invalid - meaning it is empty or has spaces.
    #[error("{self:?}")]
    InvalidWord(String),

    /// The provided text is invalid - meaning it is empty or contains only spaces
    #[error("{self:?}")]
    InvalidText(String),
}

/// Type alias for the Result enum so that callers will not need to include the error enum in it.
pub type Result<T> = core::result::Result<T, Error>;
