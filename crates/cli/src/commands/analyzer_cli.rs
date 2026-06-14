use library::{BinaryData, ExtractedModules, analyzer::analyze};

pub fn run(extracted_modules: ExtractedModules) -> BinaryData {
    let binary_data = analyze::run(extracted_modules);
    binary_data
}
