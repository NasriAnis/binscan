mod commands;
use commands::{scan, report, extract, parsing};

use library::extractor::extract::extract as lib_extract;

fn main()
{
    // parse cmd input into struct
    let args = parsing::run();
    println!("{:?}", args);

    // test only case haha
    lib_extract(args.source);
}
