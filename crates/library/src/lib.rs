mod analyzer;
pub mod extractor;
mod report;
mod sbom;
mod vuln_db;

pub struct Extracted_modules {
    pub strings: Vec<String>,
    pub symbols: Vec<String>,
    pub imports: Vec<String>,
}
