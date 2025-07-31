use super::challenge3::scoring_func;
use crate::utils::{
    file::read_lines_from_file,
    hex::{bytes_to_hex_string, hex_to_bytes},
    xor::single_byte_xor,
};
use std::{f64, path::Path};

pub fn c4() {
    let file_path = Path::new("assets/s1c4.txt");
    let lines = match read_lines_from_file(file_path) {
        Ok(lines) => lines, // returning back the lines obtained
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    // Here, we are again assuming that the best_key and the best_score
    // is a universal thing across the 60 lines of the challenge. We are
    // going to try out each key combination against each line and
    // find out the one with the highest score. (and hopefully that'll
    // decrypt things)
    let mut overall_best_score = f64::NEG_INFINITY;
    let mut overall_best_key: u8 = 0;
    let mut overall_best_plaintext_bytes: Vec<u8> = Vec::new();

    for line in lines.iter() {
        let cipherline_bytes = match hex_to_bytes(line) {
            Ok(bytes) => bytes,
            Err(e) => {
                println!("Error in text file: {}", e);
                return;
            }
        };

        let mut best_score = f64::NEG_INFINITY;
        let mut best_key: u8 = 0;
        let mut best_plaintext_bytes: Vec<u8> = Vec::new();

        // Local Check
        for candidate_key in 0..=255 {
            let candidate_plaintext_bytes = single_byte_xor(&cipherline_bytes, candidate_key);
            let current_score = scoring_func(&candidate_plaintext_bytes);
            if current_score > best_score {
                best_score = current_score;
                best_key = candidate_key;
                best_plaintext_bytes = candidate_plaintext_bytes;
            }
        }

        // Global check
        if best_score > overall_best_score {
            overall_best_score = best_score;
            overall_best_key = best_key;
            overall_best_plaintext_bytes = best_plaintext_bytes;
        }
    }

    println!("Best Key (hex): {:02x}", overall_best_key);
    if overall_best_key.is_ascii_graphic() || overall_best_key == b' ' {
        println!("Best Key (char): {}", overall_best_key as char);
    } else {
        println!("Best Key (non-printable)");
    }
    println!("Best Score: {:.2}", overall_best_score);

    // Converting resulting bytes to UTF-8 string
    match String::from_utf8(overall_best_plaintext_bytes.clone()) {
        Ok(s) => println!("Decrypted message: {}", s),
        Err(_) => {
            // Invalid UTF-8
            println!(
                "Decrypted message (invalid UTF-8), as bytes: {:?}",
                overall_best_plaintext_bytes,
            );
            println!(
                "Decrypted message (invalid UTF-8), as hex: {}",
                bytes_to_hex_string(&overall_best_plaintext_bytes),
            );
        }
    }
}
