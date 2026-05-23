use gurf_text_analyzer::{Result, TextAnalyzer, user_input};

fn main() -> core::result::Result<(), Box<dyn std::error::Error>> {
    run_text_analyzer()?;

    Ok(())
}

fn run_text_analyzer() -> Result<()> {
    let mut text_analyzer = TextAnalyzer::new();

    let text_to_add = user_input::input_string("Please Insert text for the analyzer: ")?;
    text_analyzer.add_text_to_analyzer(&text_to_add)?;

    let word_to_search =
        user_input::input_string("Please insert a word to search for frequency: ")?;
    println!(
        "the word frequency is: {}",
        text_analyzer.get_word_frequency(&word_to_search)?
    );
    Ok(())
}
