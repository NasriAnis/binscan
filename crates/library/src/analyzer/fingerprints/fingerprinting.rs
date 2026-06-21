use regex::{Regex, RegexBuilder};
use std::fs;

use crate::FileType;

pub struct FingerprintPattern {
    pub regex: Regex,
    pub package: String,
    pub ecosystem: String,
}

static ELF_PATH: &str = "crates/library/src/analyzer/fingerprints/elf_fingerprints.txt";
static PE_PATH: &str = "crates/library/src/analyzer/fingerprints/pe_fingerprints.txt";

pub fn run(file_type: FileType) -> Vec<FingerprintPattern> {
    if file_type == FileType::PE {
        read(PE_PATH)
    } else if file_type == FileType::ELF {
        read(ELF_PATH)
    } else {
        panic!()
    }
}

fn read(path: &str) -> Vec<FingerprintPattern> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
        .filter_map(|l| {
            let parts: Vec<&str> = l.splitn(3, '|').collect();
            if parts.len() < 2 {
                return None;
            }
            let regex = RegexBuilder::new(parts[0])
                .case_insensitive(true)
                .build()
                .ok()?;
            Some(FingerprintPattern {
                regex,
                package: parts[1].to_string(),
                ecosystem: parts.get(2).unwrap_or(&"").to_string(),
            })
        })
        .collect()
}
