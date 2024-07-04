use std::fs::read_to_string;
use std::path::Path;

pub fn read_kf<P: AsRef<Path>>(file_path: P) -> String {
    read_to_string(file_path).expect("Could not read file.")
}

pub fn to_bin(word: String) -> String {
    let mut bin_key = "".to_string();

    for character in word.clone().into_bytes() {
        bin_key += &format!("0{:b}", character);
    }
    bin_key
}