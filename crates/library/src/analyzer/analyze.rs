use crate::{BinaryData, ExtractedModules};
mod analyze_strings;
mod detect_compiler;

pub fn run(data: &ExtractedModules) -> BinaryData {
    let _libs = analyze_strings::run(&data);
    let _compiler = detect_compiler::run(&data);

    BinaryData {
        format: data.file_type,
        compiler: _compiler.unwrap_or("Unkown".to_string()),
        libs: _libs,
        imports: data.imports.clone(),
        security: data.security.clone(),
    }
}
