pub mod analyzer;
pub mod extractor;
mod report;
mod sbom;
mod vuln_db;

pub enum filetype {
    PE,
    ELF,
    Other,
}

pub struct Extracted_modules {
    pub file_type: filetype,
    pub strings: Vec<String>,
    pub symbols: Vec<String>,
    pub imports: Vec<String>,
}

pub struct Binary_data {
    pub binary: String,
    pub format: String,
    pub compiler: String,
    pub detected_components: Vec<String>,
    pub bahvior_flags: Vec<String>,
    pub build_info: Vec<String>,
}
