use library::db::{response, response::Response};
use owo_colors::OwoColorize;

pub fn parse(data: Vec<String>) -> Vec<Response> {
    match response::parse(data) {
        Ok(t) => t,
        Err(e) => {
            println!(
                "{}: {}",
                "Error in parsing received data from the API".red().bold(),
                e.red()
            );
            panic!()
        }
    }
}
