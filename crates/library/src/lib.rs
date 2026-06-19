pub mod analyzer;
pub mod extractor;
pub mod services;

use crate::analyzer::analyze::analyze_imports::Import;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileType {
    PE,
    ELF,
    Other,
}

#[derive(Debug, Clone, Copy)]
pub enum SecurityInfo {
    Elf(ElfSecurityInfo),
    Pe(PeSecurityInfo),
}

#[derive(Debug)]
pub struct ExtractedModules {
    pub file_type: FileType,
    pub strings: Vec<String>,
    pub imports: Vec<String>,
    pub security: SecurityInfo,
}

#[derive(Debug)]
pub struct BinaryData {
    pub format: FileType,
    pub compiler: String,
    pub libs: Vec<String>,
    pub imports: Vec<Import>,
    pub security: SecurityInfo,
    // future add ons
    // pub binary: String,
    // pub detected_components: Vec<String>,
    // pub bahvior_flags: Vec<String>,
    // pub build_info: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
pub struct ElfSecurityInfo {
    pub pie: bool,
    pub nx: bool,
    pub relro: bool,
    pub canary: bool,
    pub fortify: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct PeSecurityInfo {
    pub aslr: bool,
    pub dep: bool,
    pub cfg: bool,
    pub no_seh: bool,
    pub hi_aslr: bool,
}
