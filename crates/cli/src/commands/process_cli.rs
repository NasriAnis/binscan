use library::db::response::{self, Response, Vuln};

pub fn process(data: Vec<Response>, eco: &str) -> Vec<Vuln> {
    response::process(data, eco.to_owned())
}
