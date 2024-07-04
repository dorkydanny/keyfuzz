use std::fs::read_to_string;
use std::path::Path;
use num_bigint::BigInt;

fn read_kf<P: AsRef<Path>>(file_path: P) -> String {
    read_to_string(file_path).expect("Could not read file.")
}

fn bxor(bin_text: String, bin_key: &str) -> String{
    assert!(bin_text.bytes().all(|t| t == b'0' || t == b'1'));
    assert!(bin_key.bytes().all(|t| t == b'0' || t == b'1'));
    bin_text
        .bytes()
        .zip(bin_key.bytes().cycle())
        .map(|(t, k)| (t ^ (k & 1)) as char)
        .collect()
}

fn to_bin(word: String) -> String {
    let mut bin_key = "".to_string();

    for character in word.clone().into_bytes() {
        bin_key += &format!("0{:b}", character);
    }
    bin_key
}

fn generate_cipher(original_text: String, bin_key: String) -> String{
    let bin_text = to_bin(original_text.to_string());
    bxor(bin_text, bin_key.as_str())
}

fn generate_plaintext(cipher_text: String, bin_key: String) -> Result<String, Box<dyn std::error::Error>>{
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

    Ok(plaintext)
}

fn main() {
    let bin_key = to_bin(read_kf("src/key.kf"));
    let cipher = generate_cipher("Jake".to_string(), bin_key.clone());
    println!("{}", cipher);
    let plaintext = generate_plaintext(cipher.to_string(), bin_key.clone())?;
    println!("{}", plaintext);

}
