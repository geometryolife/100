#[derive(Default, PartialEq)]
pub enum Enum {
    #[default]
    All,
    Active,
    Completed,
}

pub struct TodoItem {
    pub title: String,
    pub active: bool,
    pub edit: bool,
}

impl TodoItem {
    pub fn singleitem(&mut self, ui: &mut eframe::egui::Ui) -> bool {
        let mut index = false;
        ui.horizontal(|ui| {
            if self.edit {
                let respon = ui.text_edit_singleline(&mut self.title);
                if respon.lost_focus() || respon.clicked_elsewhere() {
                    self.edit = false;
                }
                respon.request_focus();

                if ui.button("Delete").clicked() {
                    index = true;
                };
            } else {
                ui.checkbox(&mut self.active, &self.title);
                if ui.button("🖊").clicked(){ 
                    self.edit = true;
                };
            }
        });
        index
    }
}
