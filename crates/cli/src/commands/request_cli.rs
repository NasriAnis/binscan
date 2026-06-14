use library::{BinaryData, db::request};

pub async fn make(ecosystem: &String, binary_data: BinaryData) -> Vec<String> {
    let responses = request::make(binary_data, ecosystem.clone()).await;
    responses
}
