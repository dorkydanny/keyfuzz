mod libkf;
mod kfutils;
mod kfgen;
mod gui;

use gui::MyEguiApp;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("keyfuzz", native_options, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc))))).unwrap();
}

// original fn main()
// let cipher = libkf::generate_cipher();
// println!("{}", cipher);
// let plaintext = libkf::generate_plaintext();
// println!("{}", plaintext);
