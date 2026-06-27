use library::FileType;
mod commands;
mod user_interface;

use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;

use std::time::Duration;
use std::{path::Path, time::Instant};

use crate::user_interface::to_table;

fn main() {
    // parse cmd input into struct
    let args = commands::parsing::run();

    if Path::new(&args.source).exists() {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::with_template(" {spinner:.cyan.bold} {msg:.white}")
                .unwrap()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        );
        pb.enable_steady_tick(Duration::from_millis(80));

        pb.set_message(format!("{}", "Extracting and analyzing binary...".bold()));

        let now = Instant::now();

        let extraction_result = commands::extractor_cli::run(args.source.clone());
        let analyzer_result = commands::analyzer_cli::run(&extraction_result);

        let elapsed_time = now.elapsed();

        println!();

        to_table::draw_bindata(&analyzer_result);

        println!();
        println!(
            "{}{}{}",
            "Analyzer time: ".bold(),
            elapsed_time.as_secs().bold(),
            "s".bold()
        );

        if args.api && commands::request_cli::has_internet() {
            if extraction_result.file_type == FileType::ELF {
                pb.set_message(format!("{}", "Fetching matching CVEs from API...".bold()));

                let response = commands::request_cli::make(&args.ecosystem, analyzer_result);
                let parsed_data = commands::response_cli::parse(response);
                let processed = commands::process_cli::process(parsed_data, &args.ecosystem);

                println!();

                println!("Info :");
                for p in &processed {
                    println!(
                        "-------------------------{}-------------------------------",
                        p.id
                    );
                    let pub_date: String = match p.published.as_ref() {
                        Some(t) => t.to_string(),
                        None => "Published date not shown".to_string(),
                    };
                    let det: String = match p.details.as_ref() {
                        Some(t) => t.to_string(),
                        None => "Published date not shown".to_string(),
                    };
                    let sev: String = format!("{:?}", p.severity);

                    println!("published : {:?}", pub_date);
                    println!("details : {:?}", det);
                    println!("severity : {:?}", sev);
                }
            } else {
                println!("{}", "Cannot use API for PE files".red().bold());
                print!("-> ");
                println!("{}", "This will be implemented in futur versions".bold())
            }
        }
        pb.finish_and_clear();
        println!();
        println!("{}", "✓ Done".green().bold())
    } else {
        println!(
            "{} {}",
            "The specified path is invalid: ".yellow().bold(),
            args.source.yellow()
        );
    }
}
