pub fn single_byte_xor(cipher_text: &[u8], key: u8) -> Vec<u8> {
    cipher_text
        .iter()
        .map(|&cipher_byte| cipher_byte ^ key)
        .collect()
}
