use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "binscan", version = "1.0", about = "Binary scanner")]
pub struct ParsedArgs {
    // input file specified after -s
    #[arg(short = 's', long, help = "Path to the source file")]
    source: String,

    // saving report in the file specified after -r (optional)
    #[arg(short = 'r', long, help = "Path to save the report")]
    report: Option<String>,
}

pub fn run() -> ParsedArgs {
    ParsedArgs::parse()
}
