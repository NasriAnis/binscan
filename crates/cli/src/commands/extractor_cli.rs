use core::panic;
use library::{ExtractedModules, extractor::extract};
use owo_colors::OwoColorize;

pub fn run(source: String) -> ExtractedModules {
    let extraction_result: ExtractedModules;
    match extract::run(source) {
        Ok(t) => {
            extraction_result = t;
            extraction_result
        }
        Err(e) => {
            println!("{}: {}", "Error in extraction".red().bold(), e.red());
            panic!()
        }
    }
}
