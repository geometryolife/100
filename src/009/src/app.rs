use simple_todo::todolist::Enum;

#[derive(Default)]
pub struct MyApp {
    addtodo: String,
    task: usize,
    radio: Enum,
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        Self::default()
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My ToDo List");
            ui.add(
                eframe::egui::TextEdit::singleline(&mut self.addtodo)
                    .hint_text("Write something here"),
            );
            ui.horizontal(|ui| {
                ui.label(&self.task.to_string());
                ui.label("task left");
                ui.add_space(44.0);

                ui.selectable_value(&mut self.radio, Enum::All, "ALL");
                ui.selectable_value(&mut self.radio, Enum::Active, "Active");
                ui.selectable_value(&mut self.radio, Enum::Completed, "Completed");
            });
        });
    }
}
