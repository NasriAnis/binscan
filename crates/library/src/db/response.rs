use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct DataInResponse {
    id: String,
    #[serde(default)]
    details: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    #[serde(default)]
    vulns: Vec<DataInResponse>
}

pub fn parse(data: Vec<String>) -> Result<Vec<Response>> 
{
    let mut vec_parsed: Vec<Response> = Vec::new();
    for r in data {
        let parsed: Response = serde_json::from_str(&r)?;
        vec_parsed.push(parsed);
    }

    Ok(vec_parsed)
}