use crate::ExtractedModules;
use regex::Regex;
use std::collections::hash_set::HashSet;

pub fn run(data: &ExtractedModules) -> Vec<String> {
    let mut fetched_libs: Vec<String> = Vec::new();

    // --- ELF patterns ---
    // Versioned symbols: read@@GLIBC_2.2.5 → extract "GLIBC_2.2.5"
    let re_versioned = Regex::new(r"\w+@@(?P<lib>GLIBC_[\d.]+)").unwrap();
    // Standalone GLIBC version strings: GLIBC_2.34 (from .gnu.version_r)
    let re_glibc_ver = Regex::new(r"\bGLIBC_[\d.]+\b").unwrap();
    // Shared library sonames: libm.so.6, libssl.so.1.1, libpthread.so.0
    let re_soname = Regex::new(r"\blib[\w\-]+\.so(?:\.\d+)*\b").unwrap();
    // ld-linux / dynamic linker paths: /lib64/ld-linux-x86-64.so.2
    let re_ldlinux = Regex::new(r"\bld(?:-linux[\w\-]*)?.so(?:\.\d+)*\b").unwrap();

    // --- PE patterns ---
    // DLL names anywhere in a string: KERNEL32.dll, msvcrt.dll (case-insensitive)
    let re_dll = Regex::new(r"(?i)\b([\w\-]+\.dll)\b").unwrap();
    // Extract DLL from PE import format "FuncName|DLL.dll"
    let re_pe_import = Regex::new(r"\|([\w\-]+\.dll)$").unwrap();

    for s in data.strings.iter().chain(data.imports.iter()) {
        let s = s.as_str();

        // ELF
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

        // PE
        for caps in re_pe_import.captures_iter(s) {
            fetched_libs.push(caps[1].to_uppercase()); // normalize: kernel32.dll → KERNEL32.DLL
        }
        for caps in re_dll.captures_iter(s) {
            fetched_libs.push(caps[1].to_uppercase());
        }
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