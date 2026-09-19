// hide console on windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use st_gui2::job;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            // wsl2 issue (vs native windows os)
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../asset/1.png")[..]).unwrap(),
            )
            // removve window frame and drag
            .with_decorations(false)
            .with_transparent(true)
            .with_title("hello world")
            // modify window size
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
