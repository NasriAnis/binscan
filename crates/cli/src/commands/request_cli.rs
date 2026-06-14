use library::{BinaryData, db::request};

pub async fn make(ecosystem: &str, binary_data: BinaryData) -> Vec<String> {
    request::make(binary_data, ecosystem.to_owned()).await
}
