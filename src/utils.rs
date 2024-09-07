pub fn find_prism_path() -> Option<String> {
    let os: &str = std::env::consts::OS;
    let mut path: String;

    match os {
        "windows" => {
            path = std::env::var("LOCALAPPDATA").unwrap();
            path = format!("{}\\Programs\\PrismLauncher\\prismlauncher.exe", path);
        }
        "macos" => {
            path = format!("/Applications/Prism Launcher.app/Contents/MacOS/prismlauncher");
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

pub fn setup_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("assets/fonts/Rubik-Regular.ttf")),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}
