use core::panic;

use indicatif;
use library::{ExtractedModules, extractor::extract};
use owo_colors::OwoColorize;

use crate::commands::parsing::ParsedArgs;

pub fn run(args: ParsedArgs) -> ExtractedModules {
    let pb = indicatif::ProgressBar::new_spinner();
    pb.set_message("Extracting data from binary...");

    let extraction_result: ExtractedModules;
    match extract::run(args.source) {
        Ok(t) => {
            extraction_result = t;
            pb.finish();
            extraction_result
        }
        Err(e) => {
            pb.finish();
            println!("{}: {}", "Error in extraction".red().bold(), e.red());
            panic!()
        }
    }
}
