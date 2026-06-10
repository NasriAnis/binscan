pub fn extract(buffer: &[u8]) -> Vec<String>
{
    // loop throught the buffer byte and store
    // ASCII characteres into a temporary Vec
    let mut temp: Vec<u8> = Vec::new();
    let mut result: Vec<String> = Vec::new();
    for byte in buffer {
        if byte.is_ascii_graphic() || *byte == b' ' {
            temp.push(*byte);
        }
        // check lenght
        else {
            if temp.len() >= 6 {
                result.push(String::from_utf8_lossy(&temp).to_string())
            }
            temp.clear();
        }
    }
   result
}
