mod config;
mod monitor;
mod ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Charger la configuration
    let _config = config::Config::load_from_file("config.toml")?;

    // Lancer la UI
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "System Monitor",
        native_options,
        Box::new(
            |_cc| -> Result<Box<dyn eframe::App>, Box<dyn std::error::Error + Send + Sync>> {
                Ok(Box::new(ui::App::new()))
            },
        ),
    )?;

    Ok(())
}
