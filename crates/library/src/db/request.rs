use crate::{BinaryData, db::{json_req, make_req}};

pub async fn make(data: BinaryData, eco: String) -> Vec<String>{
    let json_body = json_req::make(&data, eco);
    // println!("JSON body: {:?}", json_body);

    let mut responses: Vec<String> = Vec::new();
    for r in json_body {
        let res = make_req::send(r).await.unwrap();
        responses.push(res);
    }
    responses
}
