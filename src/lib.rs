use eframe::egui;
pub mod job;
mod util;

impl eframe::App for job::MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let _ctx = ui.ctx().clone();
        self.sidebar(ui);
        self.mainbar(ui);
    }
}
