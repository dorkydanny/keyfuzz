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
        .add_filter("keyfile", &["kf"])
    )
    .expect("No save path provided");
    let extension_included = path.ends_with(".kf");
    if extension_included {
        std::fs::write(path.clone(), keyfile).expect("Unable to write File");
    } else {
        std::fs::write(format!("{}.kf", path
                                                .clone()
                                                .into_os_string()
                                                .into_string()
                                                .expect("Error Converting File To String")
                                    ), keyfile).expect("Unable to write File");
    }
    path
}