use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "binscan", version = "1.0", about = "Binary scanner")]
pub struct ParsedArgs {
    // input file specified after -s
    #[arg(short = 's', long, help = "Path to the source file")]
    pub source: String,

    #[arg(short = 'A', long, help = "Check via API")]
    pub api: bool,

    #[arg(
        short = 'S',
        long,
        help = "Check for severity levels.",
        requires = "api"
    )]
    pub severity: Option<Severity>,

    #[arg(
        short = 'e',
        long,
        help = "Ecosystem, default: Debian13",
        requires = "api"
    )]
    pub ecosystem: Option<String>,

    // saving report in the file specified after -r (optional)
    #[arg(short = 'r', long, help = "Path to save the report")]
    pub report: Option<String>,
}

pub fn run() -> ParsedArgs {
    ParsedArgs::parse()
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Severity {
    NotAssigned,
    Unimportant,
    Low,
    Medium,
    High,
}
