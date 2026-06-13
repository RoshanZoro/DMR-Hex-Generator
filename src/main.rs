// DMR AES key generator: cryptographically random hex keys held only in memory.

// Hide the console window on Windows release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod clipboard;
mod config;
mod crypto;
mod security;
mod ui;

use app::KeygenApp;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([960.0, 660.0])
            .with_min_inner_size([820.0, 560.0])
            .with_title("DMR AES Key Generator")
            .with_app_id("dmr_hex_keygen"),
        ..Default::default()
    };

    eframe::run_native(
        "DMR AES Key Generator",
        native_options,
        Box::new(|cc| Ok(Box::new(KeygenApp::new(cc)))),
    )
}
