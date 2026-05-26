//! Text Analyzer module that exports API for the following functionalities:
//! 1. provide one or more texts for the analyzer
//! 2. provide a word and receive the frequency count of the word against all of the texts that were provided

use std::collections::HashMap;

use crate::{Error, Result};

/// This struct provides an API to perform the operations described in this module documentation
#[derive(Default)]
pub struct TextAnalyzer {
    /// Hashmap that will contain the the frequencies for every word against all of the texts that were provided.
    word_frequencies: HashMap<String, u32>,
}

impl TextAnalyzer {
    /// Creates a new TextAnalyzer instance with no entires.
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets a text and adds all the text's words to the frequency table.
    pub fn add_text_to_analyzer(&mut self, text: &str) -> Result<()> {
        if text.trim().is_empty() {
            return Err(Error::InvalidText(text.to_string()));
        }

        text.split_whitespace()
            .for_each(|word| self.add_word_to_frequency_list(word));

        Ok(())
    }

    /// Returns the amount of times that a word has appeared in the previously given texts.
    /// NOTE: Returns an error if the word did not appear in any of the texts.
    pub fn get_word_frequency(&self, word: &str) -> Result<u32> {
        if word.is_empty() || word.contains(char::is_whitespace) {
            return Err(Error::InvalidWord(word.to_string()));
        }

        let frequency = *self
            .word_frequencies
            .get(word)
            .ok_or(Error::NonExistingWord(word.to_string()))?;

        Ok(frequency)
    }

    // Checks if a word is in the frequency list does the follwing:
    // 1. Creates a new entry in the list with frequency: 1 if the word did not exist in the frequency list.
    // 2. Adds 1 to the frequency of the word if it already exists in the frequency list
    fn add_word_to_frequency_list(&mut self, word: &str) {
        *self.word_frequencies.entry(word.to_string()).or_default() += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SINGLE_WORD: &str = "Gurf";
    const REPEATED_WORD: &str = "Gurfson Gurfson Gurfson";
    const TEXT: &str = "Gurfi Gurfland Gurfson";
    const MIXED_CASE_TEXT: &str = "Gurf gurf";
    const EMPTY_TEXT: &str = "";
    const MULTIPLE_SPACES_TEXT: &str = "Kipod    Itzik";

    fn create_analyzer(texts: &[&str]) -> Result<TextAnalyzer> {
        let mut analyzer = TextAnalyzer::new();
        for text in texts {
            analyzer.add_text_to_analyzer(text)?;
        }

        Ok(analyzer)
    }

    #[test]
    fn new_analyzer_has_no_words() {
        let analyzer = TextAnalyzer::new();
        assert_eq!(analyzer.word_frequencies.len(), 0);
    }

    #[test]
    fn single_word_text_has_frequency_one() -> Result<()> {
        let analyzer = create_analyzer(&[SINGLE_WORD])?;
        assert_eq!(analyzer.word_frequencies.len(), 1);
        assert_eq!(analyzer.get_word_frequency("Gurf")?, 1);

        Ok(())
    }

    #[test]
    fn repeated_word_in_same_text_add_frequency() -> Result<()> {
        let analyzer = create_analyzer(&[REPEATED_WORD])?;
        assert_eq!(analyzer.word_frequencies.len(), 1);
        assert_eq!(analyzer.get_word_frequency("Gurfson")?, 3);

        Ok(())
    }

    #[test]
    fn word_add_frequency_across_multiple_texts() -> Result<()> {
        let analyzer = create_analyzer(&[TEXT, REPEATED_WORD])?;
        assert_eq!(analyzer.get_word_frequency("Gurfson")?, 4);
        assert_eq!(analyzer.get_word_frequency("Gurfi")?, 1);
        assert_eq!(analyzer.get_word_frequency("Gurfland")?, 1);

        Ok(())
    }

    #[test]
    fn frequency_is_case_sensitive() -> Result<()> {
        let analyzer = create_analyzer(&[MIXED_CASE_TEXT])?;
        assert_eq!(analyzer.get_word_frequency("Gurf")?, 1);
        assert_eq!(analyzer.get_word_frequency("gurf")?, 1);

        Ok(())
    }

    #[test]
    fn non_existing_word_fails() -> Result<()> {
        let analyzer = create_analyzer(&[TEXT])?;
        let result = analyzer.get_word_frequency("notexist");
        assert!(matches!(result, Err(Error::NonExistingWord(_))));

        Ok(())
    }

    #[test]
    fn empty_text_fails() -> Result<()> {
        let result = create_analyzer(&[EMPTY_TEXT]);
        assert!(matches!(result, Err(Error::InvalidText(_))));

        Ok(())
    }

    #[test]
    fn multiple_spaces_does_not_affect() -> Result<()> {
        let analyzer = create_analyzer(&[MULTIPLE_SPACES_TEXT])?;
        assert_eq!(analyzer.get_word_frequency("Kipod")?, 1);
        assert_eq!(analyzer.get_word_frequency("Itzik")?, 1);

        Ok(())
    }

    #[test]
    fn frequency_on_word_with_spaces_fails() -> Result<()> {
        let analyzer = create_analyzer(&[TEXT])?;
        let result = analyzer.get_word_frequency("Gurf son");
        assert!(matches!(result, Err(Error::InvalidWord(_))));

        Ok(())
    }
}
