#[derive(Default)]
pub struct MyApp {}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        Self::default()
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        // todo
    }
}
