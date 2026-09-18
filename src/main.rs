use eframe::egui;
use st_gui2::job;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_resizable(true)
            .with_inner_size([1200.0, 750.0])
            .with_min_inner_size([900.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "CA Analytics",
        native_options,
        Box::new(|cc| Ok(Box::new(job::MyApp::new(&cc.egui_ctx)))),
    )
}
