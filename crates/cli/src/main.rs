use std::path::Path;
mod commands;

#[tokio::main]
async fn main()
{
    // parse cmd input into struct
    let args = commands::parsing::run();
    if Path::new(&args.source).exists(){

        commands::extractor_cli::run(args);
    }
    else {
        println!("The specified path is invalid: {}", args.source);
    }
}
