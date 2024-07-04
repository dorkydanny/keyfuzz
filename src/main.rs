use std::fs::read_to_string;
use std::path::Path;

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
    return bin_key
}

fn main() {
    let bin_text = to_bin("Jake".to_string());
    let bin_key = to_bin(read_kf("src/key.kf"));

    let cipher = bxor(bin_text, bin_key.as_str());
    let plaintext = bxor(cipher, bin_key.as_str());
    

    println!("{}", plaintext);

}
