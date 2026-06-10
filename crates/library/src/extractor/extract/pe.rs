use goblin::pe::PE;

use crate::PeSecurityInfo;

const IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA: u16 = 0x0020;
const IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE:    u16 = 0x0040;
const IMAGE_DLLCHARACTERISTICS_NX_COMPAT:       u16 = 0x0100;
const IMAGE_DLLCHARACTERISTICS_NO_SEH:          u16 = 0x0400;
const IMAGE_DLLCHARACTERISTICS_GUARD_CF:        u16 = 0x4000;

pub fn extract(buffer: &[u8]) -> (Vec<String>, Vec<String>, PeSecurityInfo)
{
    ( extract_pe_imports(buffer), extract_pe_symbols(buffer), extract_pe_security(buffer) )
}

fn extract_pe_symbols(buffer: &[u8]) -> Vec<String> {
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

fn extract_pe_imports(buffer: &[u8]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let pe = PE::parse(buffer).unwrap();

    // DLL dependencies
    for lib in &pe.libraries {
        result.push(lib.to_string());
    }

    result
}

pub fn extract_pe_security(buffer: &[u8]) -> PeSecurityInfo {
    let pe = PE::parse(buffer).unwrap();
    let chars = pe.header.optional_header.unwrap()
        .windows_fields.dll_characteristics;

    let aslr    = chars & IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE != 0;
    let dep     = chars & IMAGE_DLLCHARACTERISTICS_NX_COMPAT != 0;
    let cfg     = chars & IMAGE_DLLCHARACTERISTICS_GUARD_CF != 0;
    let no_seh  = chars & IMAGE_DLLCHARACTERISTICS_NO_SEH != 0;
    let hi_aslr = chars & IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA != 0;

    PeSecurityInfo { aslr, dep, cfg, no_seh, hi_aslr }
}