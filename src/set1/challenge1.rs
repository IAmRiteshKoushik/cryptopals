use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use hex;

pub fn c1() {
    println!("Set: 1; Challenge: 1");
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    match hex_to_base64(input) {
        Ok(base64_result) => {
            println!("Conversion output: {}", base64_result);
            println!("Expected output: {}", output);
            println!("Output comparison: {}", base64_result == output)
        }
        Err(e) => {
            println!("Error converting hex to Base64: {}", e)
        }
    }
}

pub fn hex_to_base64(input: &str) -> Result<String, hex::FromHexError> {
    // Manual way to handle the error propagation and the success case
    // let decode_result: Result<Vec<u8>, FromHexError> = hex::decode(input);
    // match decode_result {
    //     Ok(bytes) => {
    //         let base64_output = BASE64_STANDARD.encode(&bytes);
    //         Ok(base64_output)
    //     }
    //     Err(e) => Err(e),
    // }

    // This is the smarter way to handle it. The "?" operator would
    // automatically propagate the error as Err(FromHexError) without any
    // explicit error handling requirement
    let bytes = hex::decode(input)?;
    let base64_output = BASE64_STANDARD.encode(&bytes);
    Ok(base64_output)
}
