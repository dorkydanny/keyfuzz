mod libkf;
mod kfutils;
mod kfgen;

use eframe::egui;

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
           ui.heading("Hello World!");
           let generate_keyfile = ui.button("Generate Keyfuzz File");
           if generate_keyfile.clicked() {
            println!("Genkey clicked");
            kfgen::generate_kf(500);
           }
           let generate_cipher = ui.button("Generate Cipher");
           if generate_cipher.clicked() {
                println!("Cipher clicked");
                libkf::generate_cipher();
           }
       });
   }
}

// original fn main()
// let cipher = libkf::generate_cipher();
// println!("{}", cipher);
// let plaintext = libkf::generate_plaintext();
// println!("{}", plaintext);
