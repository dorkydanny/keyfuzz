use rfd::FileDialog;
use std::path::PathBuf;

pub fn open_seed() -> PathBuf{
    FileDialog::new()
    .add_filter("seedfile", &["kfs"])
    .set_directory(".")
    .pick_file()
    .unwrap_or(PathBuf::new())
}

pub fn open_plainfile() -> PathBuf{
    let path = FileDialog::new()
    .add_filter("plaintext", &["emkv", "mkv"])
    .set_directory(".")
    .pick_file()
    .unwrap_or(PathBuf::new());
    path
}