pub fn c5() {
    let plaintext = b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key = b"ICE";
    let expected_output = r"0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

    let ciphertext = repeating_key_xor(plaintext, key);
    // 1. This converts raw-bytes into a String. Vec<u8> holds the encrypted bytes
    // 2. We use an iterator which is refereing &u8 to each byte.
    // 3. Using a map function, we get access to a closure which can be applied to
    // each of the elements of the vector. (closure = lambda)
    // 4. The format! macro returns the string after formatting
    //    :x -> format it in lowercase hexadecimal (because output is 0...9a..e)
    //    :0 -> pad with leading zeros
    //    :2 -> ensure the output string for each bye is exactly two chars long
    // The padding and constant length setup is done because we always read two
    // hex-characters and convert them into a single byte.
    // Eg: "1B" -> [0x1B] and not [0x01], [0x0B]
    let actual_output: String = ciphertext
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect();
    if actual_output == expected_output {
        println!("Ciphertext matches expected output.")
    } else {
        println!("Ciphertext DOES NOT match expected output.");
        println!("Expected: {}", expected_output);
        println!("Actual:   {}", actual_output);
    }
}

fn repeating_key_xor(plaintext: &[u8], key: &[u8]) -> Vec<u8> {
    let mut ciphertext = Vec::new();
    let key_len = key.len();

    // After chaining numerate function, the iterator is returning:
    // (usize, &u8). Remember, that we are not getting access to the u8
    // because the iterator yields references to the element but not the
    // element itself. This needs to be destructured in the for-loop.
    for (i, &plaintext_byte) in plaintext.iter().enumerate() {
        // While there is implicit deferencing and you don't actually need `&`
        // However, it is best to be explicit
        let key_byte_index = i % key_len;
        let key_byte = key[key_byte_index];
        let xor_byte = plaintext_byte ^ key_byte;

        ciphertext.push(xor_byte);
    }
    ciphertext
}
