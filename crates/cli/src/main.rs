use std::path::Path;

mod commands;
use commands::parsing;
use library::{analyzer::analyze, extractor::extract, vuln_db::vuln_database::scan_db_for};

#[tokio::main]
async fn main()
{
    // parse cmd input into struct
    let args = parsing::run();
    if Path::new(&args.source).exists(){
        
        println!("Used argument: {:?}", args);
        
        let extraction_result = extract::run(args.source).expect("ERROR IN EXTRACTION");
        // debuging purposes :
        // println!("Extraction result : {:?}", extraction_result);
        
        let analyzer_result = analyze::run(extraction_result);
        // debuging purposes :
        println!("Analyzer results: {:?}", analyzer_result);

        scan_db_for().await;

        println!("Program will exit succesfully");
    }
    else {
        println!("The specified path is invalid: {}", args.source);
    }
}
