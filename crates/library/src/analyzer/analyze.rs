use crate::{BinaryData, ExtractedModules, analyzer::fingerprints::fingerprinting};
mod analyze_strings;
mod detect_compiler;

// struct VersionMatch {
//     package: String,
//     version: String,
//     ecosystem: String,
// }

pub fn run(data: &ExtractedModules) -> BinaryData {
    let fing = fingerprinting::run(data.file_type);
    let _libs = analyze_strings::run(data, &fing);
    let _compiler = detect_compiler::run(data);

    // println!("STrings : {:?}", data.strings);
    // println!("Result : {:?}", _libs);

    BinaryData {
        format: data.file_type,
        compiler: _compiler.unwrap_or("Unkown".to_string()),
        libs: _libs,
        imports: data.imports.clone(),
        security: data.security,
    }
}
