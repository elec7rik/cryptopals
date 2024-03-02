// XOR of 2 Hex strings

use std::error::Error;

extern crate hex;

fn fixed_xor(hex_string_1: &str, hex_string_2: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    // Decode the hex strings
    let bytes1: Vec<u8> = hex::decode(hex_string_1)?;
    let bytes2: Vec<u8> = hex::decode(hex_string_2)?;

    // Performing XOR
    let xor_string: Vec<u8> = bytes1.iter().zip(bytes2.iter()).map(|(a, b)| a ^ b).collect();
    Ok(xor_string)

}
fn main() {
    let hex_string_1: &str = "1c0111001f010100061a024b53535009181c";
    let hex_string_2: &str = "686974207468652062756c6c277320657965";
    match fixed_xor(hex_string_1, hex_string_2) {
        Ok(xor_string) => for byte in &xor_string {
            print!("{:02x}", byte);
        },
        Err(err) => eprintln!("Error: {}", err),
    }
    
}
