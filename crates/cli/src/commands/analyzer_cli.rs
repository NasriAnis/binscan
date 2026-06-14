use library::{BinaryData, ExtractedModules, analyzer::analyze};

pub fn run(extracted_modules: ExtractedModules) -> BinaryData {
    analyze::run(extracted_modules)
}
