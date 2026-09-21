use eframe::egui;

#[derive(PartialEq)]
enum Job {
    Mas,
    Sgx,
    Weekly,
}

impl Job {
    fn run(&self) {
        match self {
            Self::Mas => println!("run mas job"),
            Self::Sgx => println!("run sgx job"),
            Self::Weekly => println!("run weekly job"),
        }
    }
}

struct MyApp {
    job: Job,
    status: bool,
    data: Vec<String>,
    selected: usize,
    num: i32,
    color: [f32; 3],
    tab: Job,
}

impl MyApp {
    fn new(_ctx: &eframe::CreationContext) -> Self {
        Self {
            job: Job::Mas,
            status: false,
            data: vec!["laptop".to_owned(), "car".to_owned(), "travel".to_owned()],
            selected: 0,
            num: 10,
            color: [0.4, 0.6, 1.0],
            tab: Job::Mas,
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::left("sidebar").min_size(150.0).show(ui, |ui| {
            for x in 0..self.data.len() {
                if ui
                    .selectable_label(x == self.selected, &self.data[x])
                    .clicked()
                {
                    self.selected = x;
                }
            }

            ui.separator();
            ui.selectable_value(&mut self.tab, Job::Mas, "Mas");
            ui.selectable_value(&mut self.tab, Job::Sgx, "Sgx");
            ui.selectable_value(&mut self.tab, Job::Weekly, "Weekly");
        });

        egui::CentralPanel::default().show(ui, |ui| {
            // 1. Code text
            ui.monospace("head(mtcars, 3)");

            // 2. Radio button
            ui.separator();
            ui.radio_value(&mut self.job, Job::Mas, "Mas");
            ui.radio_value(&mut self.job, Job::Sgx, "Sgx");
            ui.radio_value(&mut self.job, Job::Weekly, "Weekly");

            if ui.checkbox(&mut self.status, "Show status").changed() && self.status {
                self.job.run();
                self.tab.run();
            }

            // 3. Selectable
            ui.separator();
            ui.label(&self.data[self.selected])
                .on_hover_ui_at_pointer(|ui| {
                    ui.button("World").on_hover_text("hello world");
                });

            // 4. ScrollArea
            ui.separator();
            let text_style = egui::TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);
            // let row_height = ui.spacing().interact_size.y;
            let total_rows = 3;
            egui::ScrollArea::vertical()
                .stick_to_bottom(true)
                .auto_shrink(true)
                .show_rows(ui, row_height, total_rows, |ui, row_range| {
                    for row in row_range {
                        let text = format!("Row {}/{}", row + 1, total_rows);
                        ui.label(text);
                    }
                });

            // 5. Sense
            ui.separator();
            let (rec, res) = ui.allocate_exact_size(egui::vec2(40.0, 20.0), egui::Sense::hover());
            ui.painter().rect_filled(rec, 4.0, egui::Color32::RED);
            if res.hovered() {
                ui.label("don't touch me");
            }

            // 6. Key event
            ui.input(|x| {
                if x.key_pressed(egui::Key::ArrowUp) {
                    self.num += 1;
                }
                if x.key_pressed(egui::Key::ArrowDown) {
                    self.num -= 1;
                }
            });

            // 7. Color picker
            ui.color_edit_button_rgb(&mut self.color);
            ui.label(
                egui::RichText::new(format!("you are {}", self.num))
                    .color(egui::Color32::from_rgb(
                        (self.color[0] * 255.0) as u8,
                        (self.color[1] * 255.0) as u8,
                        (self.color[2] * 255.0) as u8,
                    ))
                    .strong(),
            );
        });
    }
}

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Hello",
        native_options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}
