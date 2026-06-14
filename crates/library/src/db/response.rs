use serde::{Deserialize, Serialize};
use serde_json::Result;

// Top-level response: { "vulns": [...] }
#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    #[serde(default)]
    pub vulns: Vec<Vuln>,
}

// One vulnerability entry
#[derive(Serialize, Deserialize, Debug)]
pub struct Vuln {
    pub id: String,
    #[serde(default)]
    pub details: Option<String>,
    #[serde(default)]
    pub modified: Option<String>,
    #[serde(default)]
    pub published: Option<String>,
    #[serde(default)]
    pub upstream: Vec<String>,      // upstream CVE IDs e.g. ["CVE-2023-4911"]
    #[serde(default)]
    pub affected: Vec<Affected>,
    #[serde(default)]
    pub severity: Vec<Severity>,    // CVSS scores
}

// One affected package block
#[derive(Serialize, Deserialize, Debug)]
pub struct Affected {
    pub package: Package,
    #[serde(default)]
    pub ranges: Vec<Range>,
    #[serde(default)]
    pub versions: Vec<String>,      // explicit affected versions list
    #[serde(default)]
    pub ecosystem_specific: Option<EcosystemSpecific>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    pub name: String,
    pub ecosystem: String,          // e.g. "Debian:12"
    #[serde(default)]
    pub purl: Option<String>,
}

// Version range: introduced → fixed
#[derive(Serialize, Deserialize, Debug)]
pub struct Range {
    #[serde(rename = "type")]
    pub range_type: String,         // "ECOSYSTEM"
    #[serde(default)]
    pub events: Vec<RangeEvent>,
}

// Either { "introduced": "0" } or { "fixed": "2.31-13" }
#[derive(Serialize, Deserialize, Debug)]
pub struct RangeEvent {
    pub introduced: Option<String>,
    pub fixed: Option<String>,
}

// Debian urgency metadata
#[derive(Serialize, Deserialize, Debug)]
pub struct EcosystemSpecific {
    pub urgency: Option<String>,    // "unimportant" | "low" | "medium" | "high" | "not yet assigned"
}

// CVSS score block
#[derive(Serialize, Deserialize, Debug)]
pub struct Severity {
    #[serde(rename = "type")]
    pub severity_type: String,      // "CVSS_V3" | "CVSS_V4"
    pub score: String,              // "CVSS:3.1/AV:N/AC:L/..."
}

pub fn parse(data: Vec<String>) -> Result<Vec<Response>> {
    let mut vec_parsed: Vec<Response> = Vec::new();
    for r in data {
        let parsed: Response = serde_json::from_str(&r)?;
        vec_parsed.push(parsed);
    }
    Ok(vec_parsed)
}

pub fn process(data: Vec<Response>, eco: String) -> Vec<Vuln>{
    let mut processed: Vec<Vuln> = Vec::new();

    for res in data {
        'br: for vuln in res.vulns {
            'ct: for affect in &vuln.affected {
                if affect.package.ecosystem != eco {
                    continue 'ct;
                }
                if let Some(eco_specif) = &affect.ecosystem_specific {
                    if eco_specif.urgency == Some("unimportant".to_string())
                        || eco_specif.urgency == Some("medium".to_string())
                            || eco_specif.urgency == Some("not yet assigned".to_string())
                                || eco_specif.urgency== Some("low".to_string()) {
                        continue 'ct;
                    }
                    processed.push(vuln);
                    break 'br;
                }
            }
        }
    }
    processed
}