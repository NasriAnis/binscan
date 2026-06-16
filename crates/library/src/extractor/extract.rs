use crate::ExtractedModules;
use crate::{FileType, SecurityInfo};
use goblin::{Object, error};
use std::fs;

mod elf;
mod pe;
mod string;

pub fn run(path_to_buf: String) -> Result<ExtractedModules, error::Error> {
    // Reads the entire contents of a file into a bytes vector.
    let buffer = match fs::read(path_to_buf) {
        Ok(t) => t,
        _ => {
            return Err(error::Error::Malformed(
                "Reading the specified buffer".to_string(),
            ));
        }
    };

    let extracted_symbols: Option<Vec<String>>;
    let extracted_imports: Vec<String>;
    let extracted_security_info: SecurityInfo;
    let _file_type: FileType;

    let extracted_strings = string::extract(&buffer);

    // match binary type using goblin
    match Object::parse(&buffer)? {
        Object::Elf(_) => {
            _file_type = FileType::ELF;
            let (imports, symbols, security) = elf::extract(&buffer);
            extracted_imports = imports;
            extracted_symbols = Some(symbols);
            extracted_security_info = SecurityInfo::Elf(security);
        }

        Object::PE(_) => {
            _file_type = FileType::PE;
            let (imports, security) = pe::extract(&buffer);
            extracted_imports = imports;
            extracted_symbols = None;
            extracted_security_info = SecurityInfo::Pe(security);
        }

        _ => return Err(error::Error::Malformed("Unsupported file type".to_string())),
    }

    // saving data into public structure
    let extracted = ExtractedModules {
        file_type: _file_type,
        strings: extracted_strings,
        imports: {
            let mut combined = extracted_imports;
            combined.extend(extracted_symbols.unwrap_or_default());
            combined
        },
        security: extracted_security_info,
    };

    // debuging purpose
    // println!("Extracted Strings : \n{:?}\n\n Extracted imports : \n{:?}",
    //             extracted.strings, extracted.imports);

    Ok(extracted)
}
