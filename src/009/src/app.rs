use simple_todo::todolist::{Enum, TodoItem};

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct MyApp {
    addtodo: String,
    task: usize,
    radio: Enum,
    data: Vec<TodoItem>,
    del: i32,
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        let mut data = Self::default();
        if let Some(storage) = cc.storage {
            data = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        // simple_todo::setup_custom_fonts(&cc.egui_ctx);
        data.del = -1;
        data
    }

    fn show_content(&mut self, ui: &mut eframe::egui::Ui) {
        match self.radio {
            Enum::All => {
                self.show_all(ui);
            }
            Enum::Active => {
                self.show_active(ui, false);
            }
            Enum::Completed => {
                self.show_active(ui, true);
            }
        }
    }

    fn show_all(&mut self, ui: &mut eframe::egui::Ui) {
        if self.data.len() > 0 {
            for (idx, todoitem) in &mut self.data.iter_mut().enumerate() {
                if todoitem.singleitem(ui) {
                    self.del = idx as i32;
                };
            }
        } else {
            ui.label("You have not create task yet:ヾ(•ω•`)o");
        }
    }

    fn show_active(&mut self, ui: &mut eframe::egui::Ui, active: bool) {
        let datae = self
            .data
            .iter()
            .filter(|u| u.active == active)
            .collect::<Vec<_>>();

        if datae.len() > 0 {
            for (idx, todoitem) in &mut self.data.iter_mut().enumerate() {
                if todoitem.active == active {
                    if todoitem.singleitem(ui) {
                        self.del = idx as i32;
                    };
                }
            }
        } else {
            if active {
                ui.label("You have not task todo yet!");
            } else {
                ui.label("You have not complete yet!");
            }
        }
    }

    fn dataupdate(&mut self) {
        let datae = self
            .data
            .iter()
            .filter(|u| u.active == false)
            .collect::<Vec<_>>();
        self.task = datae.len();
        if self.del > -1 {
            self.data.remove(self.del as usize);
            self.del = -1;
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My ToDo List");
            let p = ui.add(
                eframe::egui::TextEdit::singleline(&mut self.addtodo)
                    .hint_text("Write something here"),
            );
            if p.lost_focus() & !p.clicked_elsewhere() {
                if self.addtodo.is_empty() {
                } else {
                    self.data.push(TodoItem {
                        title: self.addtodo.clone(),
                        active: false,
                        edit: false,
                    });

                    self.addtodo.clear();
                }
            }
            ui.horizontal(|ui| {
                ui.label(&self.task.to_string());
                ui.label("task left");
                ui.add_space(44.0);

                ui.selectable_value(&mut self.radio, Enum::All, "ALL");
                ui.selectable_value(&mut self.radio, Enum::Active, "Active");
                ui.selectable_value(&mut self.radio, Enum::Completed, "Completed");
            });
            ui.add_space(11.0);
            self.show_content(ui);
        });
        self.dataupdate();
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        eframe::set_value(_storage, eframe::APP_KEY, self);
    }
}
