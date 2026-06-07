use std::fs;
use goblin::{Object, error};
use crate::{SecurityInfo, FileType};

use crate::{ExtractedModules,
            extractor::extract::string::string_extraction,
            extractor::extract::elf::extract_elf_data,
            extractor::extract::pe::extract_pe_data};

mod string;
mod elf;
mod pe;

pub fn run(path_to_buf: String) ->  Result<ExtractedModules, error::Error>
{
    // Reads the entire contents of a file into a bytes vector.
    let buffer;
    match fs::read(path_to_buf) {
        Ok(t) => buffer = t,
        _ => {
            return Err(error::Error::Malformed("Unsupported file type".to_string()))
        }
    };

    let mut extracted_strings: Vec<String> = vec!["".to_string()];
    let mut extracted_symbols: Vec<String> = vec!["".to_string()];
    let mut extracted_imports: Vec<String> = vec!["".to_string()];
    let extracted_security_info: SecurityInfo;
    let _file_type: FileType;

    extracted_strings = string_extraction(&buffer);

    // match binary type using goblin
    match Object::parse(&buffer)?
    {
        Object::Elf(_) => {
            _file_type = FileType::ELF;
            let (imports, symbols, sec) = extract_elf_data(&buffer);
            extracted_imports = imports;
            extracted_symbols = symbols;
            extracted_security_info = SecurityInfo::Elf(sec);
        },

        Object::PE(_) => {
            _file_type = FileType::PE;
            let (imports, symbols, sec) = extract_pe_data(&buffer);
            extracted_imports = imports;
            extracted_symbols = symbols;
            extracted_security_info = SecurityInfo::Pe(sec);
        },

        _ => {
            return Err(error::Error::Malformed("Unsupported file type".to_string()))
        }
    }

    // saving data into public structure
    let extracted = ExtractedModules {
        file_type: _file_type,
        strings: extracted_strings,
        symbols: extracted_symbols,
        imports: extracted_imports,
        security: extracted_security_info,
    };

    // debuging purpose
    // println!("Extracted Strings : \n{:?}\n\n extracted symbols : \n{:?}\n\n Extracted imports : \n{:?}",
    //             extracted.strings, extracted.symbols, extracted.imports);

    Ok(extracted)
}