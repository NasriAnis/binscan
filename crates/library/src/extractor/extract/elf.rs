use goblin::elf::Elf;

pub fn extract_elf_symbols (buffer: &[u8]) -> Vec<String>
{
    let mut result: Vec<String> = Vec::new();

    let elf = Elf::parse(buffer).unwrap();
    // Symbols extraction
    for sym in &elf.syms {
        let name = elf.strtab.get_at(sym.st_name).unwrap_or("<unknown>");
        // println!( "{:<30}", name);
        result.push(name.to_string());
    }

    // Dynamic symbols extraction
    for sym in &elf.dynsyms {
        let name = elf.dynstrtab.get_at(sym.st_name).unwrap_or("<unknown>");

        // Undefined symbols (st_shndx == 0) are *imported* (needed from shared libs)
        // Defined symbols are *exported*
        let kind = if sym.st_shndx == goblin::elf::section_header::SHN_UNDEF as usize {
            "IMPORT"
        } else {
            "EXPORT"
        };

        // println!("[{kind}] {name}");
        result.push(kind.to_string()+name);
    }

    result
}

pub fn extract_elf_imports(buffer: &[u8]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    let elf = Elf::parse(buffer).unwrap();

    // Libraries this binary depends on extraction
    for lib in &elf.libraries {
        // println!("needs: {lib}");
        result.push(lib.to_string());
    }
    result

    // let imports = elf
    //     .dynsyms
    //     .iter()
    //     .filter(|sym| sym.st_shndx == goblin::elf::section_header::SHN_UNDEF as usize)
    //     .filter_map(|sym| elf.dynstrtab.get_at(sym.st_name))
    //     .filter(|name| !name.is_empty())
    //     .map(String::from)
    //     .collect();
}
