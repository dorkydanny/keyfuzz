use std::fmt::Write;
use rfd::FileDialog;
use rand::{distributions::Alphanumeric, Rng};
use std::path::PathBuf;

pub fn generate_kf(length: u64) -> PathBuf{
    let mut keyfile: String = String::new();
    for _ in 1..=length {
        let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
        write!(&mut keyfile, "{}", s).unwrap();
    }
    let path = FileDialog::save_file(
        FileDialog::new()
        .add_filter("plaintext", &["txt"])
    )
    .expect("No save path provided");
    std::fs::write(path.clone(), keyfile).expect("Unable to write File");
    path
}