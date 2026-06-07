pub mod analyzer;
pub mod extractor;
pub mod vuln_db;

#[derive(Debug, Clone, Copy)]
pub enum FileType {
    PE,
    ELF,
    Other,
}

#[derive(Debug)]
pub enum SecurityInfo {
    Elf(ElfSecurityInfo),
    Pe(PeSecurityInfo),
}

#[derive(Debug)]
pub struct ExtractedModules {
    pub file_type: FileType,
    pub strings: Vec<String>,
    pub symbols: Vec<String>,
    pub imports: Vec<String>,
    pub security: SecurityInfo,
}

#[derive(Debug)]
pub struct BinaryData {
    pub format: FileType,
    pub compiler: String,
    pub libs: Vec<String>,
    pub imports: Vec<String>,
    pub security: SecurityInfo,

    // future add ons
    // pub binary: String,
    // pub detected_components: Vec<String>,
    // pub bahvior_flags: Vec<String>,
    // pub build_info: Vec<String>,
}

#[derive(Debug)]
pub struct ElfSecurityInfo {
    pub pie: bool,
    pub nx: bool,
    pub relro: bool,
    pub canary: bool,
    pub fortify: bool,
}

#[derive(Debug)]
pub struct PeSecurityInfo {
    pub aslr: bool,
    pub dep: bool,
    pub cfg: bool,
    pub no_seh: bool,
    pub hi_aslr: bool,
}