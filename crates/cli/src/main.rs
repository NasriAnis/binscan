use std::{iter::Scan, path::Path};

mod commands;
// use commands::{scan, report, extract, parsing};
use commands::parsing;
use library::{analyzer::analyze, extractor::extract};

fn main()
{
    // parse cmd input into struct
    let args = parsing::run();
    if Path::new(&args.source).exists(){
        
        println!("Used argument: {:?}", args);
        
        let extraction_result = extract::run(args.source).expect("ERROR IN EXTRACTION");
        // debuging purposes :
        // println!("Extraction result : {:?}", extraction_result);
        
        let analyzer_result = analyze::run(&extraction_result);
        // debuging purposes :
        println!("Analyzer results: {:?}", analyzer_result);

        println!("Program will exit succesfully");
    }
    else {
        println!("The specified path is invalid: {}", args.source);
    }
}
