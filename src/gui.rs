use egui::{widgets, Color32, FontFamily, FontId, Image, Pos2, RichText, Shape, TextureId};
use egui::{Rect, Stroke, Widget};
use egui_extras;
use crate::{kfgen, libkf};

#[derive(Default)]
pub struct MyEguiApp {
    genkey_labeltext: RichText,
    encrypt_labeltext: RichText,
    decrypt_labeltext: RichText,
}

impl MyEguiApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
       MyEguiApp::sidebar(self, ctx);
   }
}

impl MyEguiApp {
    fn central_panel(&mut self, ctx: &egui::Context) {
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
                match libkf::crypt() {
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
                match libkf::crypt() {
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
    
    fn sidebar(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("my_left_panel")
            .min_width(200.0)
            .resizable(false)
            .show(ctx, |ui| {
                let stroke = Stroke::new(2.0, Color32::RED);
            });
        }
}