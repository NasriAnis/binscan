use crate::ExtractedModules;
use regex::Regex;

struct CompilerPattern {
    name: &'static str,
    regex: &'static str,
}

static COMPILER_PATTERNS: &[CompilerPattern] = &[
    CompilerPattern {
        name: "GCC",
        regex: r"GCC: \(.*?\) (?P<version>[\d.]+)",
    },
    CompilerPattern {
        name: "Clang",
        regex: r"clang version (?P<version>[\d.]+)",
    },
    CompilerPattern {
        name: "Rustc",
        regex: r"rustc version (?P<version>[\d.]+)",
    },
    CompilerPattern {
        name: "MSVC",
        regex: r"C/C\+\+ Optimizing Compiler Version (?P<version>[\d.]+)",
    },
    CompilerPattern {
        name: "IAR",
        regex: r"IAR C/C\+\+ Compiler V(?P<version>[\d.]+)",
    },
    CompilerPattern {
        name: "armcc",
        regex: r"armcc (?P<version>[\d.]+)",
    },
    CompilerPattern {
        name: "TCC",
        regex: r"TCC (?P<version>[\d.]+)",
    },
];

pub fn run(data: &ExtractedModules) -> Option<String> {
    // compile all patterns once
    let compiled: Vec<(&str, Regex)> = COMPILER_PATTERNS
        .iter()
        .map(|p| (p.name, Regex::new(p.regex).unwrap()))
        .collect();

    for s in &data.strings {
        for (name, re) in &compiled {
            if let Some(caps) = re.captures(s) {
                let version = &caps["version"];
                return Some(format!("{} {}", name, version));
            }
        }
    }
    None // no compiler string found (stripped or firmware)
}
