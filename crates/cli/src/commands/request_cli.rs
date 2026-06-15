use library::{BinaryData, db::request};
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};

pub async fn make(ecosystem: &str, binary_data: BinaryData) -> Vec<String> {
    request::make(binary_data, ecosystem.to_owned()).await
}

pub async fn has_internet() -> bool {
    timeout(Duration::from_secs(3), TcpStream::connect("1.1.1.1:53"))
        .await
        .map(|r| r.is_ok())
        .unwrap_or(false)
}