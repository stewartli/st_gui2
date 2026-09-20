use eframe::egui;

#[derive(Default, PartialEq)]
enum Job {
    #[default]
    Mas,
    Sgx,
    Weekly,
}

#[derive(Default)]
struct MyApp {
    checked: bool,
    search: String,
    job: Job,
    num: i32,
}

impl MyApp {
    fn show(&self, ui: &egui::Ui) {
        let ctx = ui.ctx().clone();
        // ctx.egui_wants_keyboard_input();
        // egui::Visuals::dark().panel_fill;
        if self.checked {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }
    }
    fn run(&self) {
        match self.job {
            Job::Mas => println!("job is mas"),
            Job::Sgx => println!("job is sgx"),
            Job::Weekly => println!("job is weekly"),
        }
        if self.search.contains("@") {
            println!("yes, it is email");
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Frame::new()
            .fill(egui::Color32::CYAN)
            .inner_margin(20.0)
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label("CA Analytics");
                })
            });

        egui::CentralPanel::default().show(ui, |ui| {
            // 1. checkbox like
            if ui.selectable_label(self.checked, "pick me").clicked() {
                self.checked = !self.checked;
            }

            // 2. text input
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("search");
                let res1 = ui.text_edit_singleline(&mut self.search);
                if res1.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.search.clear();
                }
            });

            // 3. radiobox
            let r1 = ui.radio_value(&mut self.job, Job::Mas, "Mas");
            let r2 = ui.radio_value(&mut self.job, Job::Sgx, "Sgx");
            let r3 = ui.radio_value(&mut self.job, Job::Weekly, "Weekly");

            if r1.changed() || r2.changed() || r3.changed() {
                self.run();
            }

            // 4. slider
            ui.add(egui::Slider::new(&mut self.num, 0..=10).text("Number"));
            ui.add_sized(
                [100.0, 40.0],
                egui::Button::new("submit").fill(egui::Color32::GREEN),
            );

            // 5. keycode
            ui.input(|i| {
                for event in &i.events {
                    if let egui::Event::Key {
                        key,
                        pressed: true,
                        repeat: false,
                        modifiers,
                        ..
                    } = event
                    {
                        println!("{key:?} mods={modifiers:?}");
                    }
                }
            });

            // 6. scope
            ui.scope(|ui| {
                ui.visuals_mut().override_text_color = Some(egui::Color32::RED);
                ui.label("become red now");
            })
        });

        self.show(ui);
    }
    fn on_exit(&mut self) {
        println!("num result => {}", self.num);
    }
}

fn main() -> eframe::Result {
    // WAYLAND_DISPLAY= cargo run --example demo1
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 300.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Hello",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}
