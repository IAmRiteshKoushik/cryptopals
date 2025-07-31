fn hex_to_bytes(hex: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    for i in (0..hex.len()).step_by(2) {
        let byte_str = &hex[i..i + 2];
        bytes.push(u8::from_str_radix(byte_str, 16).expect("Invalid max string"));
    }
    bytes
}
