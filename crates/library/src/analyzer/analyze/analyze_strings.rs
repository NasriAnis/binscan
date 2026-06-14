use crate::ExtractedModules;
use regex::Regex;
use std::collections::hash_set::HashSet;

pub fn run(data: &ExtractedModules) -> Vec<String> {
    let mut fetched_libs: Vec<String> = Vec::new();

    // 1. versioned symbols:  read@@GLIBC_2.2.5
    let re_versioned = Regex::new(r"(?P<func>\w+)@@(?P<lib>GLIBC_[\d.]+)").unwrap();
    // 2. standalone version strings:  GLIBC_2.34  (from .gnu.version_r)
    let re_glibc_ver = Regex::new(r"\bGLIBC_[\d.]+\b").unwrap();
    // 3. shared library sonames:  libc.so.6  libpthread.so.0  libm.so.6
    // let re_soname = Regex::new(r"\blib[\w\-]+\.so(?:\.[\d]+)*\b").unwrap();

    for s in data.strings.iter().chain(data.symbols.iter()) {
        let s = s.as_str();

        for caps in re_versioned.captures_iter(s) {
            fetched_libs.push(caps["lib"].to_string());
        }
        for m in re_glibc_ver.find_iter(s) {
            fetched_libs.push(m.as_str().to_string());
        }
        // for m in re_soname.find_iter(s) {
        //     fetched_libs.push(m.as_str().to_string());
        // }
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
