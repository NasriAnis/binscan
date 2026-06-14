use library::db::response::{self, Response, Vuln};

pub fn process(data: Vec<Response>, eco: &String) -> Vec<Vuln> {
    let parsed_data = response::process(data, eco.clone());

    parsed_data
}
