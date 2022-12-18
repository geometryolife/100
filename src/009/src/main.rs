mod app;

fn main() {
    // Setting the Light theme other options by default
    let option = eframe::NativeOptions {
        default_theme: eframe::Theme::Light,
        ..Default::default()
    };

    // Starting app
    eframe::run_native(
        "Simple ToDo App",
        option,
        Box::new(|cc| Box::new(app::MyApp::new(cc))),
    );
}
