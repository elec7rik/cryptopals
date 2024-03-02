// Hex to Base64 program

extern crate hex;
extern crate base64;

use std::error::Error;

fn hex_to_base64(hex_string: &str) -> Result<String, Box<dyn Error>> {
    // Decode the hex string
    let bytes: Vec<u8> = hex::decode(hex_string)?;

    // Encode bytes to base64
    let base64_string: String = base64::encode(&bytes);

    Ok(base64_string)
}

fn main() {
    let hex_string: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";    //Example string
    match hex_to_base64(hex_string) {
        Ok(base64_string) => println!("Base64 representation: {}", base64_string),
        Err(err) => eprintln!("Error: {}", err),
    }

}

