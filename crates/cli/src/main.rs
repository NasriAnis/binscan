use std::path::Path;
use commands::parsing;
use library::{analyzer::analyze, extractor::extract, db::{request, response}};

mod commands;

#[tokio::main]
async fn main()
{
    // parse cmd input into struct
    let args = parsing::run();
    if Path::new(&args.source).exists(){
        
        println!("Used argument: {:?}", args);
        
        let extraction_result = extract::run(args.source).expect("ERROR IN EXTRACTION");
        // println!("Extraction result : {:?}", extraction_result); // debuging purposes
        
        let analyzer_result = analyze::run(extraction_result);
        // println!("Analyzer results: {:?}", analyzer_result); // debuging purposes

        let responses = request::make(analyzer_result, "Debian:12".to_string()).await;
        // println!("Responses: {:?}", responses);
        
        let parsed_data = response::parse(responses);
        println!("Parsed Data: {:?}", parsed_data);
        
        println!("Program will exit succesfully");
    }
    else {
        println!("The specified path is invalid: {}", args.source);
    }
}
