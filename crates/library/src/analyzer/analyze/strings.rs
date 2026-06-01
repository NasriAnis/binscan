use regex::Regex;
use std::collections::hash_set::HashSet;
use crate::Extracted_modules;

struct CompilerPattern {
    name  : &'static str,
    regex : &'static str,
}

static COMPILER_PATTERNS: &[CompilerPattern] = &[
    CompilerPattern {
        name  : "GCC",
        regex : r"GCC: \(.*?\) (?P<version>[\d.]+)",
    },
    CompilerPattern {
        name  : "Clang",
        regex : r"clang version (?P<version>[\d.]+)",
    },
    CompilerPattern {
        name  : "Rustc",
        regex : r"rustc version (?P<version>[\d.]+)",
    },
    CompilerPattern {
        name  : "MSVC",
        regex : r"C/C\+\+ Optimizing Compiler Version (?P<version>[\d.]+)",
    },
    CompilerPattern {
        name  : "IAR",
        regex : r"IAR C/C\+\+ Compiler V(?P<version>[\d.]+)",
    },
    CompilerPattern {
        name  : "armcc",
        regex : r"armcc (?P<version>[\d.]+)",
    },
    CompilerPattern {
        name  : "TCC",
        regex : r"TCC (?P<version>[\d.]+)",
    },
];

pub fn analyze_strings(data: &Extracted_modules) -> (Vec<String>, String)
{
    let re = Regex::new(r"(?P<functions>\w+)@@(?P<libs>[\w.]+)").unwrap();
    let mut fetched_libs: Vec<String> = Vec::new();

    for strings in &data.strings {
        let Some(caps) = re.captures(strings.as_str()) else {
            continue;
        };
        // println!("The lib is: {} {}", &caps["function"], &caps["lib"]);
        fetched_libs.push(caps["libs"].to_string());
    }
    fetched_libs = dedup_strings(fetched_libs);

    let compiler: Option<String> = detect_compiler(&data.strings);

    (fetched_libs, compiler.unwrap_or("Unkown".to_string()))
}

fn dedup_strings(strings: Vec<String>) -> Vec<String>
{
    let mut seen = HashSet::new();
    strings.into_iter()
        .filter(|s| seen.insert(s.clone()))
        .collect()
}

pub fn detect_compiler(strings: &[String]) -> Option<String>
{
    // compile all patterns once
    let compiled: Vec<(&str, Regex)> = COMPILER_PATTERNS
        .iter()
        .map(|p| (p.name, Regex::new(p.regex).unwrap()))
        .collect();

    for s in strings {
        for (name, re) in &compiled {
            if let Some(caps) = re.captures(s) {
                let version = &caps["version"];
                return Some(format!("{} {}", name, version));
            }
        }
    }
    None   // no compiler string found (stripped or firmware)
}
