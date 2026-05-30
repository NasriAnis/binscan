use crate::{Binary_data, Extracted_modules, analyzer::analyze::strings::analyze_strings_for_libs};
mod strings;

pub fn analyze(data: &Extracted_modules) //-> Binary_data
{
    let mut libs: Vec<String> = Vec::new();
    let mut compiler = String::new();
    (libs, compiler) = analyze_strings_for_libs(data);
}
