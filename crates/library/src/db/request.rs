use tokio::time::{Duration, sleep};

use crate::{
    BinaryData,
    db::{json_req, make_req},
};

pub async fn make(data: BinaryData, eco: String) -> Vec<String> {
    let json_body = json_req::make(&data, eco);
    // println!("JSON body: {:?}", json_body);

    let mut responses: Vec<String> = Vec::new();

    for r in json_body {
        let mut result: Option<String> = None;

        for attempt in 1..=3 {
            match make_req::send(&r).await {
                Ok(t) => {
                    result = Some(t);
                    break;
                }
                Err(e) => {
                    eprintln!("[attempt {}/3] Request failed: {}", attempt, e);
                    sleep(Duration::from_millis(500)).await;
                }
            }
        }

        match result {
            Some(t) => responses.push(t),
            None => eprintln!("All 3 attempts failed for: {}", r),
        }
    }
    responses
}
