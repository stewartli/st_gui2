use eframe::egui;

use crate::util;

const SIB_BG: egui::Color32 = egui::Color32::from_rgb(22, 22, 25);

#[allow(unused)]
pub struct MyApp {
    status: Vec<i32>,
    pub message: String,
    checked: bool,
}

impl MyApp {
    pub fn new(ctx: &egui::Context) -> Self {
        // 1. reset theme
        ctx.set_visuals(egui::Visuals::dark());
        // 2. reset visuals
        /*
        ctx.request_repaint_after_secs(100.0);
        ctx.set_debug_on_hover(true);
        */
        // 3. reset font
        let mut myfont = egui::FontDefinitions::default();
        myfont.font_data.insert(
            "my_font".to_owned(),
            egui::FontData::from_static(include_bytes!("../asset/LavishlyYours-Regular.ttf"))
                .into(),
        );
        myfont
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "my_font".to_owned());
        myfont
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push("my_font".to_owned());
        ctx.set_fonts(myfont);
        // 4. reset style
        ctx.all_styles_mut(|x| {
            x.spacing.item_spacing = egui::vec2(10.0, 10.0);
            x.spacing.button_padding = egui::vec2(14.0, 8.0);
            x.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(35, 35, 45);
            x.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(85, 55, 150);
            x.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(65, 45, 115);
            // x.text_styles.insert(
            //     TextStyle::Heading,
            //     FontId::new(20.0, egui::FontFamily::Monospace),
            // );
            x.text_styles.insert(
                egui::TextStyle::Body,
                egui::FontId::new(12.0, egui::FontFamily::Monospace),
            );
        });

        Self {
            status: vec![1, 2, 3, 4],
            message: String::new(),
            checked: false,
        }
    }
    /*
    // eframe = { version = "0.29", features = ["persistence"] }
    // serde = { version = "1", features = ["derive"] }
    fn new1(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(x) = cc.storage {
            eframe::get_value(x, eframe::APP_KEY).unwrap()
        } else {
            Self::new(&cc.egui_ctx)
        }
    }
    */
    pub fn sidebar(&mut self, ui: &mut egui::Ui) {
        egui::Panel::left("sidebar")
            // .exact_size(320.0)
            .resizable(true)
            .default_size(320.0)
            .size_range(180.0..=420.0)
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

                ui.button("Open file")
                    .on_hover_text("rfd")
                    .clicked()
                    .then(|| println!("open rfd"));

                // 3. TextEdit
                ui.separator();
                ui.add(egui::Separator::default().spacing(20.0));
                ui.style_mut().visuals.hyperlink_color = egui::Color32::CYAN;

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

                // 5. Link
                if ui.link("Document").clicked() {
                    println!("thank you for link");
                }

                ui.hyperlink_to("google", "www.google.com");
                ui.add(egui::github_link_file!(
                    "https://github.com/stewartli/st_gui2/blob/main/src/job.rs",
                    "Source code"
                ));
            });
    }
    pub fn mainbar(&mut self, ui: &mut egui::Ui) {
        // global theme + egui command
        egui::Panel::top("menu bar").show(ui, |ui| {
            egui::menu::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Exit").clicked() {
                        ui.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ui, |ui| {
            // 1. Collapsing
            ui.collapsing("News", |ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.label("name");
                    ui.text_edit_singleline(&mut self.message);
                })
            });

            // 2. ComboBox
            let choice = ["test1", "test2", "test3"];
            egui::ComboBox::from_label("Select job")
                .selected_text("pick some")
                .show_ui(ui, |ui| {
                    for (i, x) in choice.iter().enumerate() {
                        if ui
                            .selectable_value(&mut Some(0), Some(i), choice[0])
                            .clicked()
                        {
                            println!("{i}, {x}");
                        }
                    }
                });

            // 2.1 Tab
            let mut pick = false;
            ui.horizontal(|ui| {
                pick |= ui.selectable_value(&mut self.status[0], 1, "mas").changed();
                pick |= ui.selectable_value(&mut self.status[0], 2, "stx").changed();
            });

            if pick {
                match self.status[0] {
                    1 => println!("do mas job"),
                    2 => println!("do sgx job"),
                    _ => println!("do other job"),
                }
            }

            // 3. ScrollArea
            egui::ScrollArea::vertical()
                .id_salt("view scroll")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.label("R scripts");
                    ui.add_space(20.0);

                    // 3.1. Sense
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

                    // Sense::drag
                    ui.allocate_exact_size(egui::vec2(20.0, 20.0), egui::Sense::click_and_drag());

                    let panel_rect = ui.allocate_rect(ui.min_rect(), egui::Sense::drag());
                    if panel_rect.hovered() || panel_rect.hovered() || panel_rect.dragged() {
                        println!("hover or drag area");
                    }

                    // 3.2 TextEdit
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

                    // 3.3 Layout
                    ui.add_space(20.0);
                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.label("btn1");
                            ui.label("btn2");
                            ui.label("btn3");
                        })
                    });

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
                    );

                    // 3.4 painter
                    ui.painter().circle_stroke(
                        ui.available_rect_before_wrap().center(),
                        12.0,
                        egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 0, 0)),
                    );

                    let (resp, pat) =
                        ui.allocate_painter(egui::vec2(120.0, 120.0), egui::Sense::hover());
                    let center = resp.rect.center();
                    pat.circle_stroke(center, 50.0, egui::Stroke::new(2.0, egui::Color32::WHITE));

                    // 3.5 Window
                    egui::Window::new("help me")
                        .anchor(egui::Align2::CENTER_BOTTOM, [0.0, 0.0])
                        .collapsible(true)
                        .resizable(true)
                        .movable(true)
                        .open(&mut self.message.is_empty())
                        .show(ui, |ui| {
                            if ui.button("win-btn").clicked() {
                                println!("win-btn clicked");
                            }
                        })
                })
        });
    }
    pub fn footbar(&self, ui: &mut egui::Ui) {
        egui::Panel::bottom("foot bar")
            .resizable(false)
            .min_size(0.0)
            .show(ui, |ui| {
                ui.label("CA Analytics - Audit Data Analytics");
            });
    }
}
