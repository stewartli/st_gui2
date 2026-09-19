use eframe::egui;
pub mod job;
mod util;

impl eframe::App for job::MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // let _ctx = ui.ctx().clone();
        // ui.request_repaint();
        // _frame.storage();
        self.sidebar(ui);
        self.mainbar(ui);
        self.footbar(ui);
    }
    fn on_exit(&mut self) {
        println!("my app => {}", self.message);
    }
}
