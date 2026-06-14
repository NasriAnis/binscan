use library::{analyzer::analyze, db::request, db::response};
use owo_colors::OwoColorize;
use std::path::Path;
mod commands;

#[tokio::main]
async fn main() {
    // parse cmd input into struct
    let args = commands::parsing::run();
    if Path::new(&args.source).exists() {
        let extraction_result = commands::extractor_cli::run(args);

        let analyzer_result = analyze::run(extraction_result);
        // println!("Analyzer results: {:?}", analyzer_result); // debuging purposes

        let responses = request::make(analyzer_result, "Debian:12".to_string()).await;
        // println!("Responses: {:?}", responses);

        let parsed_data = response::parse(responses).unwrap();
        // println!("Parsed Data: {:?}", parsed_data);

        let processed = response::process(parsed_data, "Debian:12".to_string());
        println!("Processed data : {processed:?}");
    } else {
        println!(
            "{}: {}",
            "The specified path is invalid".yellow().bold(),
            args.source.yellow()
        );
    }
}
