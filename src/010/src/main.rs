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
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Add").clicked() {
                self.value += 1;
            };
            ui.label(self.value.to_string());
            if ui.button("Reduce").clicked() {
                self.value -= 1;
            };
        });
    }
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
