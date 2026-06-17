use library::FileType;
use owo_colors::OwoColorize;
use std::path::Path;
mod commands;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

#[tokio::main]
async fn main() {
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

        let extraction_result = commands::extractor_cli::run(args.source.clone());
        let analyzer_result = commands::analyzer_cli::run(&extraction_result);

        println!();

        println!("Data from binary:");
        println!("Compiler: {}", analyzer_result.compiler);
        println!("Format: {:?}", analyzer_result.format);
        println!("Imports: {:?}", analyzer_result.imports);
        println!("Libs: {:?}", analyzer_result.libs);
        println!("Security info: {:?}", analyzer_result.security);

        println!();

        if args.api && commands::request_cli::has_internet().await {
            if extraction_result.file_type == FileType::ELF {
                pb.set_message(format!("{}", "Fetching matching CVEs from API...".bold()));

                let response = commands::request_cli::make(&args.ecosystem, analyzer_result).await;
                let parsed_data = commands::response_cli::parse(response);
                let processed = commands::process_cli::process(parsed_data, &args.ecosystem);

                println!();

                // let mut seen = HashSet::new();
                // processed.retain(|p| seen.insert(p.id.clone()));

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
