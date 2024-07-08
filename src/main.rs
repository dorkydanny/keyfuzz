mod libkf;
mod kfutils;
mod kfgen;

use eframe::egui;
use egui::{widgets, Color32, FontFamily, FontId, Image, RichText};
use egui_extras;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("keyfuzz", native_options, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc))))).unwrap();
}

#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
            egui_extras::install_image_loaders(ctx);
            ui.heading("KeyFuzz");
            let path = std::env::current_dir().unwrap();
            let keygenimg = Image::new(format!("file://{}/src/assets/keygen.png", path.to_str().unwrap()));
            let seed_label = RichText::new("Generate Seed").font(FontId::new(24.0, FontFamily::Proportional)).color(Color32::WHITE);
            ui.heading(seed_label);
            if ui.add_sized([80.0, 80.0], widgets::ImageButton::new(keygenimg)).clicked() {
                println!("Image button clicked");
                println!("Genkey clicked");
                let mut genkey_labeltext = String::from("");
                match kfgen::generate_seed() {
                    Ok(_) => genkey_labeltext = String::from("Success"),
                    Err(e) => genkey_labeltext = e.to_string(),
                };
                ui.label(genkey_labeltext);
            }
            let encryptimg = Image::new(format!("file://{}/src/assets/encrypt.png", path.to_str().unwrap()));
            let encrypt_label = RichText::new("Encrypt File").font(FontId::new(24.0, FontFamily::Proportional)).color(Color32::WHITE);
            ui.heading(encrypt_label);
            if ui.add_sized([80.0, 80.0], widgets::ImageButton::new(encryptimg)).clicked() {
                println!("Encrypt clicked");
                    let result = libkf::generate_cipher();
                    //ui.label(result);
            }
            let decryptimg = Image::new(format!("file://{}/src/assets/decrypt.png", path.to_str().unwrap()));
            let decrypt_label = RichText::new("Decrypt File").font(FontId::new(24.0, FontFamily::Proportional)).color(Color32::WHITE);
            ui.heading(decrypt_label);
            if ui.add_sized([80.0, 80.0], widgets::ImageButton::new(decryptimg)).clicked() {
                println!("Decrypt clicked");
                    let result = libkf::generate_cipher();
                    //ui.label(result);
            }
       });
   }
}

// original fn main()
// let cipher = libkf::generate_cipher();
// println!("{}", cipher);
// let plaintext = libkf::generate_plaintext();
// println!("{}", plaintext);
