use egui::{FontId, RichText};

pub struct GeronimoApp {
    prism_launcher_path: String,
}

impl Default for GeronimoApp {
    fn default() -> Self {
        Self {
            prism_launcher_path: "".to_string(),
        }
    }
}

pub fn find_prism_path() -> Option<String> {
    let os: &str = std::env::consts::OS;
    let mut path: String;

    match os {
        "windows" => {
            path = std::env::var("LOCALAPPDATA").unwrap();
            path = format!("{}/Programs/Prism Launcher/Prism Launcher.exe", path);
        }
        "macos" => {
            path = format!("/Applications/Prism Launcher.app");
        }
        _ => {
            return None;
        }
    }
    if std::path::Path::new(&path).exists() {
        Some(path)
    } else {
        None
    }
}

impl eframe::App for GeronimoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(
                    RichText::new("Geronimo Manager")
                        .font(FontId::proportional(60.0))
                        .color(egui::Color32::from_rgb(0, 0, 0)),
                );
            });

            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.prism_launcher_path);
            });
        });
    }
}
