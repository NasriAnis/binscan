use goblin::pe::PE;
use pelite::pe32::Pe as Pe32;
use pelite::pe32::imports::Import as Import32;
use pelite::pe64::Pe as Pe64;
use pelite::pe64::imports::Import as Import64;
use std::panic;

use crate::PeSecurityInfo;

const IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA: u16 = 0x0020;
const IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE: u16 = 0x0040;
const IMAGE_DLLCHARACTERISTICS_NX_COMPAT: u16 = 0x0100;
const IMAGE_DLLCHARACTERISTICS_NO_SEH: u16 = 0x0400;
const IMAGE_DLLCHARACTERISTICS_GUARD_CF: u16 = 0x4000;

pub fn extract(buffer: &[u8]) -> (Vec<String>, PeSecurityInfo) {
    let import_result = match extract_pe_imports(buffer) {
        Ok(t) => t,
        Err(_) => panic!(),
    };

    (import_result, extract_pe_security(buffer))
}

fn extract_pe_imports(buffer: &[u8]) -> Result<Vec<String>, pelite::Error> {
    let mut result = Vec::new();

    // Try PE64 first, fall back to PE32
    if let Ok(pe) = pelite::pe64::PeFile::from_bytes(buffer) {
        for desc in pe.imports()? {
            let dll = desc.dll_name()?;
            for import in desc.int()? {
                match import? {
                    Import64::ByName { name, .. } => {
                        result.push(format!("{}|{}", name, dll));
                    }
                    Import64::ByOrdinal { ord } => {
                        result.push(format!("{}|{}", ord, dll));
                    }
                }
            }
        }
    } else if let Ok(pe) = pelite::pe32::PeFile::from_bytes(buffer) {
        for desc in pe.imports()? {
            let dll = desc.dll_name()?;
            for import in desc.int()? {
                match import? {
                    Import32::ByName { name, .. } => {
                        result.push(format!("{}|{}", name, dll));
                    }
                    Import32::ByOrdinal { ord } => {
                        result.push(format!("{}|{}", ord, dll));
                    }
                }
            }
        }
    }
    Ok(result)
}

pub fn extract_pe_security(buffer: &[u8]) -> PeSecurityInfo {
    let pe = PE::parse(buffer).unwrap();
    let chars = pe
        .header
        .optional_header
        .unwrap()
        .windows_fields
        .dll_characteristics;

    let aslr = chars & IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE != 0;
    let dep = chars & IMAGE_DLLCHARACTERISTICS_NX_COMPAT != 0;
    let cfg = chars & IMAGE_DLLCHARACTERISTICS_GUARD_CF != 0;
    let no_seh = chars & IMAGE_DLLCHARACTERISTICS_NO_SEH != 0;
    let hi_aslr = chars & IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA != 0;

    PeSecurityInfo {
        aslr,
        dep,
        cfg,
        no_seh,
        hi_aslr,
    }
}
