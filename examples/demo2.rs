use eframe::egui;

#[derive(Default, PartialEq)]
enum JobKind {
    #[default]
    Daily,
    Weekly,
    Monthly,
}

#[derive(Default)]
struct MyApp {
    kind: JobKind,
    job: String,
    theme: bool,
    out: String,
}

impl std::fmt::Display for JobKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Self::Daily => "daily",
            Self::Weekly => "weekly",
            Self::Monthly => "monthly",
        };
        write!(f, "{res}")
    }
}

impl MyApp {
    fn new(ctx: &egui::Context) -> Self {
        ctx.all_styles_mut(|x| {
            x.text_styles.insert(
                egui::TextStyle::Heading,
                egui::FontId::new(20.0, egui::FontFamily::Monospace),
            );
        });
        Self::default()
    }
    fn theme(&self, ctx: &egui::Context) {
        ctx.set_visuals(if self.theme {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        });
    }
    fn run(&mut self) {
        let res = std::path::PathBuf::from(&self.job);
        self.job.clear();
        self.out = if res.exists() {
            format!("run {} job in {}", self.kind, res.display())
        } else {
            "no such job".into()
        };
    }
    fn topbar(&mut self, ui: &mut egui::Ui) {
        egui::Panel::top("topbar")
            .exact_size(50.0)
            .frame(
                egui::Frame::side_top_panel(ui.style())
                    .inner_margin(egui::Margin::symmetric(16, 12)),
            )
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("Audit Data Analytics");
                    ui.separator();
                    ui.label("File");
                    ui.separator();
                    if ui.selectable_label(self.theme, "Theme").clicked() {
                        self.theme = !self.theme;
                        self.theme(ui.ctx());
                    }
                })
            });
    }
    fn footbar(&self, ui: &mut egui::Ui) {
        egui::Panel::bottom("footbar")
            .frame(
                egui::Frame::side_top_panel(ui.style())
                    .inner_margin(egui::Margin::symmetric(16, 12)),
            )
            .show(ui, |ui| {
                ui.vertical_centered(|ui| ui.label("© 2026 CA Analytics"));
            });
    }
    fn mainbar(&mut self, ui: &mut egui::Ui) {
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.label("Job");
            ui.text_edit_singleline(&mut self.job);
        });

        ui.add_space(10.0);
        egui::ComboBox::from_label("Job kind")
            .selected_text(self.kind.to_string())
            .show_ui(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.kind, JobKind::Daily, "daily");
                    ui.selectable_value(&mut self.kind, JobKind::Weekly, "weekly");
                    ui.selectable_value(&mut self.kind, JobKind::Monthly, "monthly");
                });
            });

        ui.add_space(10.0);
        if ui.button("Submit").clicked() {
            self.run();
        }

        ui.add_space(10.0);
        ui.label(egui::RichText::new(&self.out).color(egui::Color32::CYAN));
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.topbar(ui);
        self.footbar(ui);
        self.mainbar(ui);
    }
}

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "CA Analytics",
        native_options,
        Box::new(|cc| Ok(Box::new(MyApp::new(&cc.egui_ctx)))),
    )
}
