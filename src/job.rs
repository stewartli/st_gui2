use eframe::egui;

use crate::util;

const SIB_BG: egui::Color32 = egui::Color32::from_rgb(22, 22, 25);

#[allow(unused)]
pub struct MyApp {
    status: i32,
    message: Option<String>,
    checked: bool,
}

impl MyApp {
    pub fn new(ctx: &egui::Context) -> Self {
        // reset theme
        ctx.request_repaint_after_secs(100.0);
        ctx.set_visuals(egui::Visuals::dark());
        ctx.all_styles_mut(|x| {
            x.spacing.item_spacing = egui::vec2(10.0, 10.0);
            x.spacing.button_padding = egui::vec2(14.0, 8.0);
            x.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(35, 35, 45);
            x.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(85, 55, 150);
            x.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(65, 45, 115);
        });
        Self {
            status: 0,
            message: None,
            checked: false,
        }
    }
    pub fn sidebar(&mut self, ui: &mut egui::Ui) {
        egui::Panel::left("sidebar")
            .exact_size(320.0)
            .resizable(true)
            .frame(egui::Frame::new().fill(SIB_BG).inner_margin(20.0))
            .show(ui, |ui| {
                // 1. Heading
                ui.add_space(10.0);
                ui.heading(egui::RichText::new("⏰ CA Analytics").size(30.0).strong());
                ui.add_space(5.0);
                ui.label(egui::RichText::new("Your tools").color(egui::Color32::ORANGE));
                ui.add_space(30.0);

                // 2. Button
                util::button(ui, "Jobs", true);

                let job_btn = egui::Button::new(
                    egui::RichText::new("New Job")
                        .size(15.0)
                        .strong()
                        .color(egui::Color32::WHITE),
                )
                .min_size(egui::vec2(180.0, 42.0))
                .fill(egui::Color32::CYAN)
                .corner_radius(8.0);
                // add widgets to ui
                if ui.add(job_btn).clicked() {
                    println!("add your tool here");
                }

                // 3. TextEdit
                ui.separator();
                let mut buf = String::new();
                ui.horizontal(|ui| {
                    ui.label("Search");
                    let txt1 = ui.add(
                        egui::TextEdit::singleline(&mut buf)
                            .desired_width(180.0)
                            .hint_text("search tool name")
                            .background_color(egui::Color32::WHITE)
                            .margin(egui::Margin::symmetric(10, 8)),
                    );
                    // Enter diff: multiline vs singleline
                    if txt1.lost_focus() && ui.input(|x| x.key_pressed(egui::Key::Enter)) {
                        println!("you are done");
                    }
                });

                // 4. Checkbox
                ui.checkbox(&mut self.checked, "checkme");
                // crash if I use both
                let check_txt = if self.checked {
                    egui::RichText::new("check me")
                        .strikethrough()
                        .color(egui::Color32::RED)
                } else {
                    egui::RichText::new("hello check")
                };
                ui.label(check_txt);
                // unlock button
                // ui.add_enabled_ui(self.checked, |ui| {
                //     if ui.button("Button that is not always clickable").clicked() {
                //         println!("waha check box");
                //     }
                // });
            });
    }
    pub fn mainbar(&self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.label("hello");
            // 1. ScrollArea
            egui::ScrollArea::vertical()
                .id_salt("view scroll")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.label("R scripts");
                    ui.add_space(20.0);

                    // 2. Sense
                    let res = egui::Frame::new()
                        .fill(egui::Color32::GRAY)
                        .corner_radius(egui::CornerRadius::same(10))
                        .inner_margin(egui::Margin::symmetric(14, 10))
                        .show(ui, |ui| {
                            ui.set_min_width(ui.available_width());
                            ui.label("more view script");
                            ui.add_space(20.0);
                        })
                        .response;
                    if res.interact(egui::Sense::click()).clicked() {
                        println!("do not touch me");
                    }

                    // 3. Layout
                    ui.add_space(20.0);
                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.label("btn1");
                            ui.label("btn2");
                            ui.label("btn3");
                        })
                    });

                    // 4. TextEdit
                    ui.add_space(20.0);
                    let mut buf1 = String::new();
                    let txt = ui.add_sized(
                        [ui.available_width(), 48.0],
                        egui::TextEdit::multiline(&mut buf1)
                            .font(egui::TextStyle::Heading)
                            .hint_text("see your script to work")
                            .background_color(egui::Color32::WHITE)
                            .margin(egui::Margin::symmetric(14, 10)),
                    );
                    // key in char changed
                    if txt.changed() {
                        println!("typed => {}", buf1);
                    }

                    // 5. Layout
                    ui.allocate_ui_with_layout(
                        egui::vec2(ui.available_width(), ui.available_height()),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            ui.add_space(20.0);
                            ui.label("topdown");
                            egui::Frame::new()
                                .fill(egui::Color32::GREEN)
                                .corner_radius(8.0)
                                .show(ui, |ui| {
                                    ui.add_space(20.0);
                                    ui.label("again");
                                })
                        },
                    )
                })
        });
    }
}
