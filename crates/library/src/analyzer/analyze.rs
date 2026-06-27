use crate::{BinaryData, ExtractedModules, analyzer::fingerprints::fingerprinting};

pub mod analyze_imports;
mod analyze_strings;
mod detect_compiler;

pub fn run(extracted_modules: &ExtractedModules) -> BinaryData {
    let libs = analyze_strings::run(&extracted_modules.strings, extracted_modules.file_type);
    let compiler = detect_compiler::run(&extracted_modules.strings);
    let imports =
        match analyze_imports::run(&extracted_modules.imports, extracted_modules.file_type) {
            Ok(t) => t,
            Err(_) => panic!(),
        };
    BinaryData {
        format: extracted_modules.file_type,
        compiler: compiler.unwrap_or("Unkown".to_string()),
        libs,
        imports,
        security: extracted_modules.security,
    }
}
