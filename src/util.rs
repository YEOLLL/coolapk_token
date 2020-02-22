pub fn hex_to_string(bytes: &Vec<u8>) -> String {
    return bytes.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join("");
}