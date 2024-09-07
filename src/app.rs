use egui::RichText;

use crate::utils::{self, find_prism_path};

pub struct GeronimoApp {
    prism_launcher_path: String,
    prism_launcher_version: String,
    geronimo_manager_version: String,
}

impl Default for GeronimoApp {
    fn default() -> Self {
        Self {
            prism_launcher_path: utils::find_prism_path().unwrap(),
            prism_launcher_version: std::process::Command::new(&utils::find_prism_path().unwrap())
                .arg("--version")
                .output()
                .expect("Failed to get Prism Launcher version")
                .stdout
                .iter()
                .map(|&c| c as char)
                .collect::<String>(),
            geronimo_manager_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

impl eframe::App for GeronimoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.style_mut().interaction.selectable_labels = false;

            ui.horizontal(|ui| {
                if find_prism_path().is_none() {
                    ui.label(
                        RichText::new("PRISM LAUNCHER NOT FOUND")
                            .color(egui::Color32::from_rgb(200, 50, 50)),
                    );
                } else {
                    ui.label(
                        RichText::new("PRISM LAUNCHER FOUND")
                            .color(egui::Color32::from_rgb(50, 200, 50)),
                    );
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                    ui.horizontal(|ui| {
                        ui.label(format!(
                            "Geronimo Manager v{}",
                            self.geronimo_manager_version
                        ));
                    });
                });
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().interaction.selectable_labels = false;

            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.heading(
                    RichText::new("Geronimo Manager")
                        .size(60.0)
                        .color(egui::Color32::from_rgb(0, 0, 0)),
                );
            });

            ui.horizontal(|ui| {
                ui.label("Prism Launcher Path:");
                ui.heading(self.prism_launcher_path.clone());
            });
            ui.add_space(10.0);
            ui.vertical(|ui| {
                ui.label("Prism Launcher Version:");
                ui.heading(self.prism_launcher_version.clone());
            });
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui.button("Open Prism Launcher").clicked() {
                    std::process::Command::new(&self.prism_launcher_path)
                        .spawn()
                        .expect("Failed to open Prism Launcher");
                }
            });
        });
    }
}
