pub struct TemplateApp {
    value: i32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self { value: 0 }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {}
}

fn main() {
    let native_options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "SimpleCounter",
        native_options,
        Box::new(|cc| Box::new(TemplateApp::new(cc))),
    );
}
