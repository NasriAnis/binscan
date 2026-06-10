use crate::BinaryData;
use serde_json::{self, json};
// use serde::{Serialize, Deserialize};

// #[derive(Serialize, Deserialize, Debug)]
// struct req_format {
//     name: String,
//     ecosystem: String,
//     version: String,
// }


pub fn make(data: &BinaryData, eco: String) -> Vec<String>{
    let mut requests: Vec<String> = Vec::new();
    
    for libs in &data.libs {
        
        let l: Vec<&str> = libs.split("_").collect();
        
        // let query = req_format {
        //     name: l[0].to_string(),
        //     version: l[1].to_string(),
        //     ecosystem: eco.clone(),
        // };

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