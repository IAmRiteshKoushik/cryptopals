use hex::FromHexError;

pub fn c2() {
    println!("Set: 1; Challenge: 2");
    let input_1 = "1c0111001f010100061a024b53535009181c";
    let input_2 = "686974207468652062756c6c277320657965";
    let output = "746865206b696420646f6e277420706c6179";
    match fixed_xor(input_1, input_2) {
        Ok(result_bytes) => {
            let result = hex::encode(result_bytes); // convert to string
            println!("XOR output: {}", result);
            println!("Expected output: {}", output);
            println!("Output comparison: {}", result == output)
        }
        Err(e) => println!("Error in program: {}", e),
    }
}

// The problem assumes that both the hex-inputs are of equal length otherwise
// it can lead to undefined behavior during iterating over the byte-buffers and
// XOR-ing them together
pub fn fixed_xor(hex_input1: &str, hex_input2: &str) -> Result<Vec<u8>, FromHexError> {
    // If there are any errors in decoding, the error is propagated upwards
    let bytes1 = hex::decode(hex_input1)?;
    let bytes2 = hex::decode(hex_input2)?;

    // Empty vector to store the XOR-ed result with capacity
    let mut xored_bytes: Vec<u8> = Vec::with_capacity(bytes1.len());

    for (b1, b2) in bytes1.iter().zip(bytes2.iter()) {
        let xor_result = b1 ^ b2;
        xored_bytes.push(xor_result);
    }
    Ok(xored_bytes)
}
