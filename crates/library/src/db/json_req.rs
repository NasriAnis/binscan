use crate::BinaryData;
use serde_json::{self, json};

// for now only works for GCC versions (elf files)

pub fn make(data: &BinaryData, eco: String) -> Vec<String> {
    let mut requests: Vec<String> = Vec::new();

    for libs in &data.libs {
        let l: Vec<&str> = libs.split("_").collect();

        if l[0] != "GCC".to_string() {
            continue;
        };

        let body = json!({
            "package": {
                "name": l[0].to_lowercase(),
                "ecosystem": eco
            }, "version": l[1]
        });

        // Serialize to JSON string
        let json = serde_json::to_string(&body).unwrap();
        requests.push(json);
    }
    requests
}
