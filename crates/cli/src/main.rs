mod commands;
use commands::{scan, report, extract, parsing};

fn main() {
    // parse cmd input into struct
    let args = parsing::run();
}
