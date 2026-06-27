pub fn send(data: &str) -> Result<String, reqwest::Error> {
    const OS_API: &str = "https://api.osv.dev/v1/query";

    // debugging purpose only (test request in a proxy):
    // let client = reqwest::Client::builder()
    //     .proxy(reqwest::Proxy::https("http://127.0.0.1:8080")?)
    //     .danger_accept_invalid_certs(true)
    //     .build()?;

    let client = reqwest::blocking::Client::new();
    client
        .post(OS_API)
        .header("Content-Type", "application/json")
        .body(data.to_owned())
        .send()?
        .text()
}
