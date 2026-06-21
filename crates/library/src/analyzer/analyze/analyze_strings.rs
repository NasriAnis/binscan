use crate::FileType;

use regex::Regex;
use std::collections::hash_set::HashSet;

pub fn run(strings: &Vec<String>, file_type: FileType) -> Vec<String> {
    let mut fetched_libs: Vec<String> = Vec::new();

    if file_type == FileType::ELF {
        // Versioned symbols: read@@GLIBC_2.2.5 → extract "GLIBC_2.2.5"
        let re_versioned = Regex::new(r"\w+@@(?P<lib>GLIBC_[\d.]+)").unwrap();
        // Standalone GLIBC version strings: GLIBC_2.34 (from .gnu.version_r)
        let re_glibc_ver = Regex::new(r"\bGLIBC_[\d.]+\b").unwrap();
        // Shared library sonames: libm.so.6, libssl.so.1.1, libpthread.so.0
        let re_soname = Regex::new(r"\blib[\w\-]+\.so(?:\.\d+)*\b").unwrap();
        // ld-linux / dynamic linker paths: /lib64/ld-linux-x86-64.so.2
        let re_ldlinux = Regex::new(r"\bld(?:-linux[\w\-]*)?.so(?:\.\d+)*\b").unwrap();

        fetched_libs = patern_find_in(strings, file_type, fetched_libs);
        for s in strings.iter() {
            let s = s.as_str();

            for caps in re_versioned.captures_iter(s) {
                fetched_libs.push(caps["lib"].to_string());
            }
            for m in re_glibc_ver.find_iter(s) {
                fetched_libs.push(m.as_str().to_string());
            }
            for m in re_soname.find_iter(s) {
                fetched_libs.push(m.as_str().to_string());
            }
            for m in re_ldlinux.find_iter(s) {
                fetched_libs.push(m.as_str().to_string());
            }
        }
    } else if file_type == FileType::PE {
        fetched_libs = patern_find_in(strings, file_type, fetched_libs);
    } else {
        panic!()
    }

    fetched_libs = dedup_strings(fetched_libs);
    fetched_libs
}

fn dedup_strings(strings: Vec<String>) -> Vec<String> {
    let mut seen = HashSet::new();
    strings
        .into_iter()
        .filter(|s| seen.insert(s.clone()))
        .collect()
}

fn patern_find_in(
    string: &Vec<String>,
    file_type: FileType,
    mut fetched_libs: Vec<String>,
) -> Vec<String> {
    let patterns = super::fingerprinting::run(file_type);
    let mut seen = HashSet::new();

    for s in string {
        for p in &patterns {
            if let Some(caps) = p.regex.captures(s) {
                let version = caps.get(1).map(|m| m.as_str()).unwrap_or("unknown");
                let key = format!("{}@{}", p.package, version);
                if seen.insert(key) {
                    let combined_string = format!("{} {} {}", p.package, version, p.ecosystem);
                    fetched_libs.push(combined_string);
                }
            }
        }
    }
    fetched_libs.to_vec()
}
