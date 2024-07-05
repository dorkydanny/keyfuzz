mod libkf;
mod kfutils;
use kfutils::read_kf;
use kfutils::to_bin;

fn main() {
    let bin_key = to_bin(read_kf("src/key.kf"));
    let cipher = libkf::generate_cipher("Jake".to_string(), bin_key.clone());
    println!("{}", cipher);
    let plaintext = libkf::generate_plaintext(cipher.to_string(), bin_key.clone());
    println!("{}", plaintext);
}
