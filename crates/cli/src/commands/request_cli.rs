use library::{BinaryData, services::request};

use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

pub fn make(ecosystem: &str, binary_data: BinaryData) -> Vec<String> {
    request::make(binary_data, ecosystem.to_owned())
}

pub fn has_internet() -> bool {
    let addr: SocketAddr = "1.1.1.1:53".parse().unwrap();
    TcpStream::connect_timeout(&addr, Duration::from_secs(3)).is_ok()
}
