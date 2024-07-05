use std::fs::read_to_string;
use std::path::Path;
use std::fmt::Write;
use rfd::FileDialog;
use std::path::PathBuf;

fn open_file() -> PathBuf{
    FileDialog::new()
    .add_filter("keyfile", &["kf"])
    .set_directory(".")
    .pick_file()
    .expect("Invalid File")
}

pub fn read_kf<P: AsRef<Path>>(file_path: P) -> String {
    open_file();
    read_to_string(file_path).expect("Could not read file.")
}

pub fn to_bin(word: String) -> String {
    let mut bin_key = "".to_string();

    for character in word.clone().into_bytes() {
        write!(bin_key, "{}", &format!("0{:b}", character)).unwrap();
    }
    bin_key
}