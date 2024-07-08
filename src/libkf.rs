use crate::kfutils;
use rfd::FileDialog;
use std::path::PathBuf;
use crate::kfgen;

fn bxor(bin_text: &mut [u8], bin_key: &[u8]) {
    for (t, &k) in bin_text.iter_mut().zip(bin_key.iter()) { 
        *t ^= k; 
    }
}

pub fn crypt() -> Result<String, Box<dyn std::error::Error>>{
    let mut data = std::fs::read(kfutils::open_plainfile())?;
    let cipher_key = kfgen::generate_kf(data.len())?;
    bxor(&mut data, &cipher_key);
    let path = FileDialog::save_file(
        FileDialog::new()
    )
    .unwrap_or(PathBuf::new());
    std::fs::write(path.clone(), data)?;
    Ok(String::from("Success"))
}

/*pub fn generate_plaintext() -> String{
    let cipher_text = read_sf();
    let path = kfutils::open_seed();
    let bin_key = to_bin(std::fs::read_to_string(path.clone()).expect("Bad Path"));
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
    println!("Delete keyfuzz file?: y/n");
    let mut delete_key = String::new();
    std::io::stdin().read_line(&mut delete_key).unwrap();
    delete_key = String::from(delete_key.trim());
    if delete_key == String::from("y") {
        remove_file(path).expect("Bad Path");
    }

    plaintext
}*/