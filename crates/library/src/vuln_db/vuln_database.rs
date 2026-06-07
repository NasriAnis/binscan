use reqwest::{Client, Response};
use std::{collections::HashMap};

pub async fn scan_db_for(){
    // body_prepare(data);
    request().await.unwrap();
}

fn body_prepare(data: Vec<String>) -> String {
    "RETURNED".to_string()
}

async fn request() -> Result<Response, reqwest::Error> 
{
    const OS_API: &str = "https://api.osv.dev/v1/query";

    let mut map = HashMap::new();
    map.insert("lang", "rust");
    map.insert("body", "json");

    let client = reqwest::Client::new();

    let res = client
        .post(OS_API)
        .json(&map)
        .build()?;
    
    println!("-----------------------API REQUEST-----------------------");
    // now you can inspect everything
    println!("URL:     {}", res.url());
    println!("Method:  {}", res.method());
    println!("Headers: {:#?}", res.headers());
    println!("Body:    {:?}", res.body());
    
    let request = client.execute(res).await;
    
    println!("-----------------------API RESPONSE-----------------------");
    println!("the response: {:?}", request);

    println!("-----------------------------------------------------------");

    request
}