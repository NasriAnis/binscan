use crate::{Binary_data, Extracted_modules, analyzer::analyze::strings::analyze_strings};
mod strings;

pub fn analyze(data: &Extracted_modules) -> Binary_data
{
    let mut _libs: Vec<String> = Vec::new();
    let mut _compiler = String::new();
    (_libs, _compiler) = analyze_strings(data);

    Binary_data {
        libs: _libs,
        compiler: _compiler,
    }
}
