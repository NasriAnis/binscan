use goblin::elf::{Elf, header::ET_DYN, program_header::{PF_X, PT_GNU_RELRO, PT_GNU_STACK}};

use crate::ElfSecurityInfo;

pub fn extract(buffer: &[u8]) -> (Vec<String>, Vec<String>, ElfSecurityInfo) {
    (
        extract_elf_imports(buffer),
        extract_elf_symbols(buffer),
        extract_elf_security(buffer),
    )
}

fn extract_elf_symbols(buffer: &[u8]) -> Vec<String> {
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
        result.push(kind.to_string() + name);
    }
    result
}

fn extract_elf_imports(buffer: &[u8]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    let elf = Elf::parse(buffer).unwrap();

    // Libraries this binary depends on extraction
    for lib in &elf.libraries {
        // println!("needs: {lib}");
        result.push(lib.to_string());
    }
    result
}

// detect PIE, NX, RELRO, stack canary, FORTIFY
fn extract_elf_security(buffer: &[u8]) -> ElfSecurityInfo {
    let elf = Elf::parse(buffer).unwrap();

    let pie = elf.header.e_type == ET_DYN;

    // NX: PT_GNU_STACK present with no exec flag
    let nx = elf
        .program_headers
        .iter()
        .any(|ph| ph.p_type == PT_GNU_STACK && (ph.p_flags & PF_X == 0));

    // RELRO: PT_GNU_RELRO segment exists
    let relro = elf
        .program_headers
        .iter()
        .any(|ph| ph.p_type == PT_GNU_RELRO);

    // canary: imports __stack_chk_fail
    let canary = elf
        .dynsyms
        .iter()
        .any(|s| elf.dynstrtab.get_at(s.st_name) == Some("__stack_chk_fail"));

    // FORTIFY: any __foo_chk symbol
    let fortify = elf.dynsyms.iter().any(|s| {
        elf.dynstrtab
            .get_at(s.st_name)
            .unwrap_or("")
            .ends_with("_chk")
    });

    ElfSecurityInfo {
        pie,
        nx,
        relro,
        canary,
        fortify,
    }
}
