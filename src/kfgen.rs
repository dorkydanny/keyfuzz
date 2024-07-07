use std::{fmt::Write, fs::read_to_string};
use egui::epaint::tessellator::Path;
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
    let mut path = FileDialog::save_file(
        FileDialog::new()
        .add_filter("keyfile", &["kf"])
    )
    .unwrap_or(PathBuf::new());
    println!("Path: {:?}", path);
    let mut err = 0;
    let extension_included = path.ends_with(".kf");
    if extension_included {
        std::fs::write(path.clone(), keyfile).unwrap_or(err = 1);
    } else {
        std::fs::write(format!("{}.kf", path
                                                .clone()
                                                .into_os_string()
                                                .into_string()
                                                .unwrap()
                                    ), keyfile).unwrap_or(err = 1);
    }
    if err == 1 {
        path = PathBuf::new();
    }
    path
}