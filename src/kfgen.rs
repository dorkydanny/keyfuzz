use rand::rngs::StdRng;
use rfd::FileDialog;
use rand::{Rng, RngCore, SeedableRng};
use std::path::PathBuf;
use crate::kfutils::open_seed;
use std::fs::File;
use std::io::Read;

pub fn generate_kf(length: usize) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut seed_path = File::open(open_seed())?;
    let mut seed = [0u8; 32];
    seed_path.read_exact(&mut seed)?;
    let mut seeded_rng = StdRng::from_seed(seed);
    let mut cipher_bin = vec![0u8; length];
    seeded_rng.fill_bytes(&mut cipher_bin);
    Ok(cipher_bin)
}

pub fn generate_seed() -> std::io::Result<()> { 
    let seed = rand::rngs::OsRng.gen::<[u8; 32]>();
    let path = FileDialog::save_file(
        FileDialog::new()
        .add_filter("seedfile", &["kfs"])
    ).ok_or("Bad save path").map_err(|err| std::io::Error::new(std::io::ErrorKind::NotFound, err))?;
    let extension_included = path.extension().is_some_and(|ext| ext == "kfs");
    let path = if extension_included { path } else { 
        PathBuf::from({
            let mut path = path.as_os_str().to_os_string();
            path.push(".kfs");
            path
        })
    };
    std::fs::write(path, seed)?;
    Ok(())
}