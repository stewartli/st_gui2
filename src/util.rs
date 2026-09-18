use eframe::egui;

pub fn button(ui: &mut egui::Ui, txt: &str, selected: bool) -> bool {
    let fill = if selected {
        egui::Color32::from_rgb(91, 55, 170)
    } else {
        egui::Color32::TRANSPARENT
    };
    ui.add_sized(
        [180.0, 42.0],
        egui::Button::new(egui::RichText::new(txt).size(12.0).strong()).fill(fill),
    )
    .clicked()
}
