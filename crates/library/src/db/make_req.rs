pub async fn send(data: String) -> Result<String, reqwest::Error> 
{
    const OS_API: &str = "https://api.osv.dev/v1/query";
    
    // debugging purpose only :
    // let client = reqwest::Client::builder()
    //     .proxy(reqwest::Proxy::https("http://127.0.0.1:8080")?)
    //     .danger_accept_invalid_certs(true)
    //     .build()?;

    let client = reqwest::Client::new();
    let response = client.post(OS_API)
        .header("Content-Type", "application/json")
        .body(data)
        .send()
        .await?
        .text()
        .await;

    response
}