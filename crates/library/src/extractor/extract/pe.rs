use goblin::pe::PE;

pub fn extract_pe_symbols(buffer: &[u8]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let pe = PE::parse(buffer).unwrap();

    // Exported symbols
    for export in &pe.exports {
        let name = export.name.unwrap_or("<unnamed>");
        result.push(format!("[EXPORT] {name}"));
    }

    // Imported symbols
    for import in &pe.imports {
        result.push(format!("[IMPORT] {} (from {})", import.name, import.dll));
    }

    result
}

pub fn extract_pe_imports(buffer: &[u8]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let pe = PE::parse(buffer).unwrap();

    // DLL dependencies
    for lib in &pe.libraries {
        result.push(lib.to_string());
    }

    result
}
