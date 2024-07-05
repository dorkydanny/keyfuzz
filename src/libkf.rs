use crate::kfutils::to_bin;

fn bxor(bin_text: String, bin_key: &str) -> String{
    assert!(bin_text.bytes().all(|t| t == b'0' || t == b'1'));
    assert!(bin_key.bytes().all(|t| t == b'0' || t == b'1'));
    bin_text
        .bytes()
        .zip(bin_key.bytes().cycle())
        .map(|(t, k)| (t ^ (k & 1)) as char)
        .collect()
}

pub fn generate_cipher(original_text: String, bin_key: String) -> String{
    let bin_text = to_bin(original_text.to_string());
    bxor(bin_text, bin_key.as_str())
}

pub fn generate_plaintext(cipher_text: String, bin_key: String) -> String{
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

    plaintext
}