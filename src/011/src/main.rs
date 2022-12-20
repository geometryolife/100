#[derive(Default)]
pub struct TemplateApp {
    input: String,
    todo: Vec<String>,
    delidx: usize,
    delmark: bool,
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            // todo: vec!["coding".to_string(), "practicing English".to_string()],
            ..Default::default()
        }
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.input);
                if ui.button("Add").clicked() {
                    self.todo.push(self.input.clone());
                }
            });
            for (idx, item) in self.todo.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(item);
                    if ui.button("Del").clicked() {
                        self.delidx = idx;
                        self.delmark = true;
                    }
                });
            }
            if self.delmark {
                self.delmark = false;
                self.todo.remove(self.delidx);
            }
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "SimpleMemo",
        native_options,
        Box::new(|cc| Box::new(TemplateApp::new(cc))),
    );
}
