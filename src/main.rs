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
struct MyEguiApp {
    genkey_labeltext: RichText,
    encrypt_labeltext: RichText,
    decrypt_labeltext: RichText,
}

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
            let seed_label = RichText::new("Generate Seed")
                                        .font(FontId::new(24.0, FontFamily::Proportional))
                                        .color(Color32::WHITE);
            ui.heading(seed_label);
            if ui.add_sized([80.0, 80.0], widgets::ImageButton::new(keygenimg)).clicked() {
                match kfgen::generate_seed() {
                    Ok(_) => self.genkey_labeltext = RichText::new("Success")
                                .font(FontId::new(12.0, FontFamily::Proportional))
                                .color(Color32::LIGHT_GREEN),
                    Err(e) => self.genkey_labeltext = RichText::from(e.to_string())
                    .font(FontId::new(12.0, FontFamily::Proportional))
                    .color(Color32::LIGHT_RED),
                };
            }
            ui.label(self.genkey_labeltext.clone());
            let encryptimg = Image::new(format!("file://{}/src/assets/encrypt.png", path.to_str().unwrap()));
            let encrypt_label = RichText::new("Encrypt File").font(FontId::new(24.0, FontFamily::Proportional)).color(Color32::WHITE);
            ui.heading(encrypt_label);
            if ui.add_sized([80.0, 80.0], widgets::ImageButton::new(encryptimg)).clicked() {
                match libkf::generate_cipher() {
                    Ok(_) => self.encrypt_labeltext = RichText::new("Success")
                                        .font(FontId::new(12.0, FontFamily::Proportional))
                                        .color(Color32::LIGHT_GREEN),
                    Err(e) => self.encrypt_labeltext = RichText::from(e.to_string())
                                                            .font(FontId::new(12.0, FontFamily::Proportional))
                                                            .color(Color32::LIGHT_RED),
                };
            }
            ui.label(self.encrypt_labeltext.clone());
            let decryptimg = Image::new(format!("file://{}/src/assets/decrypt.png", path.to_str().unwrap()));
            let decrypt_label = RichText::new("Decrypt File").font(FontId::new(24.0, FontFamily::Proportional)).color(Color32::WHITE);
            ui.heading(decrypt_label);
            if ui.add_sized([80.0, 80.0], widgets::ImageButton::new(decryptimg)).clicked() {
                match libkf::generate_cipher() {
                    Ok(_) => self.decrypt_labeltext = RichText::new("Success")
                                        .font(FontId::new(12.0, FontFamily::Proportional))
                                        .color(Color32::LIGHT_GREEN),
                    Err(e) => self.decrypt_labeltext = RichText::from(e.to_string())
                                                            .font(FontId::new(12.0, FontFamily::Proportional))
                                                            .color(Color32::LIGHT_RED),
                };
            }
            ui.label(self.decrypt_labeltext.clone());
       });
   }
}

// original fn main()
// let cipher = libkf::generate_cipher();
// println!("{}", cipher);
// let plaintext = libkf::generate_plaintext();
// println!("{}", plaintext);
