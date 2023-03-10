#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "egui-mpi2p",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    address: String,
    signature: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            address: "".to_owned(),
            signature: "".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("egui-mpi2p-v0.1.0");
            ui.horizontal(|ui| {
                let address_label = ui.label("address: ");
                ui.text_edit_singleline(&mut self.address)
                    .labelled_by(address_label.id);
            });
            ui.horizontal(|ui| {
                let signature_label = ui.label("signature: ");
                ui.text_edit_singleline(&mut self.signature)
                    .labelled_by(signature_label.id);
            });
            if ui.button("Login").clicked() {
                self.signature += " *";
            }
            ui.label(format!("address: {}, signature: {}", self.address, self.signature));
        });
    }
}
