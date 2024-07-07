use std::fs::read_to_string;
use std::fmt::Write;
use rfd::FileDialog;
use std::path::PathBuf;
use crate::kfgen::generate_kf;

pub fn open_keyfile() -> PathBuf{
    FileDialog::new()
    .add_filter("keyfile", &["kf"])
    .set_directory(".")
    .pick_file()
    .unwrap_or(PathBuf::new())
}

fn open_plainfile() -> PathBuf{
    let path = FileDialog::new()
    .add_filter("plaintext", &["txt"])
    .set_directory(".")
    .pick_file()
    .unwrap_or(PathBuf::new());
    path
}

pub fn read_kf() -> String {
    let file_path = generate_kf(50000);
    read_to_string(file_path).unwrap_or(String::new())
}

pub fn read_sf() -> String {
    let file_path = open_plainfile();
    read_to_string(file_path).unwrap_or(String::new())
}

pub fn to_bin(word: String) -> String {
    let mut bin_key = "".to_string();

    for character in word.clone().into_bytes() {
        write!(bin_key, "{}", &format!("{:08b}", character)).unwrap();
    }
    bin_key
}