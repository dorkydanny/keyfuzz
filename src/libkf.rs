use std::fs::{read_to_string, remove_file};

use crate::kfutils::{self, open_keyfile, read_sf, to_bin};
use rfd::FileDialog;

fn bxor(bin_text: String, bin_key: &str) -> String{
    assert!(bin_text.bytes().all(|t| t == b'0' || t == b'1'));
    assert!(bin_key.bytes().all(|t| t == b'0' || t == b'1'));
    bin_text
        .bytes()
        .zip(bin_key.bytes().cycle())
        .map(|(t, k)| (t ^ (k & 1)) as char)
        .collect()
}

pub fn generate_cipher() -> String{
    let original_text = read_sf();
    let cipher_key = read_to_string(open_keyfile()).unwrap_or(String::new());
    if original_text == "" || cipher_key == "" {
        return String::from("Error");
    }
    let bin_key = to_bin(cipher_key);
    let bin_text = to_bin(original_text.to_string());
    let cipherfile = bxor(bin_text, bin_key.as_str());
    let path = FileDialog::save_file(
        FileDialog::new()
        .add_filter("plaintext", &["txt"])
    )
    .expect("No save path provided");
    std::fs::write(path.clone(), cipherfile.clone()).expect("Unable to write File");
    cipherfile
}

pub fn generate_plaintext() -> String{
    let cipher_text = read_sf();
    let path = kfutils::open_keyfile();
    let bin_key = to_bin(std::fs::read_to_string(path.clone()).expect("Bad Path"));
    let byte_str = bxor(cipher_text,bin_key.as_str());
    let bytes: Vec<u8> = byte_str
    .chars()
    .collect::<Vec<char>>()
    .chunks(8)
    .map(|chunk| {
        let byte_str: String = chunk.iter().collect();
        u8::from_str_radix(&byte_str, 2).unwrap()
    })
    .collect();


    let plaintext: String = bytes.iter().map(|&b| b as char).collect();
    println!("Delete keyfuzz file?: y/n");
    let mut delete_key = String::new();
    std::io::stdin().read_line(&mut delete_key).unwrap();
    delete_key = String::from(delete_key.trim());
    if delete_key == String::from("y") {
        remove_file(path).expect("Bad Path");
    }

    plaintext
}