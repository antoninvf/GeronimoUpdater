#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

pub mod app;
pub mod utils;

use crate::app::GeronimoApp;
use eframe::egui;
use native_dialog::{MessageDialog, MessageType};

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    // check prism path on start up
    let prism_path = utils::find_prism_path();
    // if prism path is none, show error message
    if prism_path.is_none() {
        let os = std::env::consts::OS;
        let message;

        if os != "windows" && os != "macos" {
            message = format!("Unsupported OS ({os}).\nThis application is currently only supported on Windows and macOS.");
        } else {
            message = format!("Prism Launcher not found.\nPlease install Prism Launcher first.");
        }
        let dialog = MessageDialog::new()
            .set_type(MessageType::Warning)
            .set_title("Error")
            .set_text(&message)
            .show_confirm()
            .unwrap();

        if (!dialog) {
            std::process::exit(1);
        }
    } else {
        let path = prism_path.unwrap();
        println!("Prism Launcher found at: {path}");
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([940.0, 720.0])
            .with_resizable(false)
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("assets/weedcat.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "Geronimo Manager",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            // Setup fonts
            utils::setup_fonts(&cc.egui_ctx);

            Ok(Box::<GeronimoApp>::default())
        }),
    )
}
