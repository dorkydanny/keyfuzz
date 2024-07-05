mod libkf;
mod kfutils;
mod kfgen;

fn main() {
    let cipher = libkf::generate_cipher();
    println!("{}", cipher);
    let plaintext = libkf::generate_plaintext();
    println!("{}", plaintext);
}
