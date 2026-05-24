use std::fs;
use goblin::{Object, error};
use crate::filetype;

use crate::{Extracted_modules, Binary_data,
            extractor::extract::string::string_extraction,
            extractor::extract::elf::{extract_elf_imports, extract_elf_symbols},
            extractor::extract::pe::{extract_pe_imports, extract_pe_symbols}};

mod string;
mod elf;
mod pe;

pub fn extract(path_to_buf: String) ->  Result<Extracted_modules, error::Error>
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
    let _file_type: filetype;

    extracted_strings = string_extraction(&buffer);

    // match binary type using goblin
    match Object::parse(&buffer)?
    {
        Object::Elf(_) => {
            _file_type = filetype::ELF;
            extracted_symbols = extract_elf_symbols(&buffer);
            extracted_imports = extract_elf_imports(&buffer);
        },

        Object::PE(_) => {
            _file_type = filetype::PE;
            extracted_symbols = extract_pe_symbols(&buffer);
            extracted_imports = extract_pe_imports(&buffer);
        },

        _ => {
            return Err(error::Error::Malformed("Unsupported file type".to_string()))
        }
    }

    // saving data into public structure
    let extracted = Extracted_modules {
        file_type: _file_type,
        strings: extracted_strings,
        symbols: extracted_symbols,
        imports: extracted_imports,
    };

    // debuging purpose
    // println!("Extracted Strings : \n{:?}\n\n extracted symbols : \n{:?}\n\n Extracted imports : \n{:?}",
    //             extracted.strings, extracted.symbols, extracted.imports);

    Ok(extracted)
}
