/// We derive Deserialize/Serialize so we can persist app state on shutdown.
// if we add new fields, give them default values when deserializing old state
use egui::{Color32, RichText};

use crate::Pet;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
#[serde(default)]
pub struct PetApp {
    pet: Pet,
}

impl PetApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        };

        Default::default()
    }
}

impl eframe::App for PetApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { pet } = self;

        const STATUS_TEXT_SIZE: f32 = 48.;

        let dead_alive_text = if pet.is_alive() {
            RichText::new("Alive")
                .size(STATUS_TEXT_SIZE)
                .color(Color32::GREEN)
                .underline()
        } else {
            RichText::new("Dead")
                .size(STATUS_TEXT_SIZE)
                .color(Color32::RED)
                .underline()
        };

        let pet_status_text = RichText::new(format!("{} is:", pet.name)).size(STATUS_TEXT_SIZE);

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Pet Actions");
                ui.set_max_width(175.);

                ui.horizontal(|ui| {
                    ui.label("Pet Name: ");
                    ui.text_edit_singleline(&mut pet.name);
                });

                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    egui::global_dark_light_mode_buttons(ui);
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            egui::warn_if_debug_build(ui);
            ui.heading("Pet Menu");
            if pet.is_alive() {
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.set_min_width(300.);
                    ui.label(pet.status());
                    ui.add_space(10.);
                    ui.scope(|ui| {
                        ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);

                        let walk_button = ui.add_sized([120., 20.], egui::Button::new("Walk"));
                        ui.add_space(10.);
                        let feed_button = ui.add_sized([120., 20.], egui::Button::new("Feed"));
                        ui.add_space(10.);
                        let grow_button = ui.add_sized([120., 20.], egui::Button::new("Grow Up"));
                        ui.add_space(10.);

                        if walk_button.clicked() {
                            pet.walk();
                        }
                        if feed_button.clicked() {
                            pet.feed()
                        };
                        if grow_button.clicked() {
                            pet.grow_up()
                        };
                    });
                    if pet.age >= 10 {
                        ui.add_space(10.);
                        let child_button = ui.add_sized(
                            [150., 40.],
                            egui::Button::new(
                                RichText::new("Have Child").heading().color(Color32::BLACK),
                            )
                            .fill(Color32::GREEN),
                        );
                        ui.add_space(10.);
                        if child_button.clicked() {
                            let old_pet_name = pet.name.to_string();
                            let older_children = pet.children.clone();
                            *pet = Pet::default();

                            for child in older_children {
                                pet.adopt_child(child)
                            }

                            pet.adopt_child(old_pet_name);
                        };
                    };
                });
            }
            ui.separator();
            ui.add_space(20.);
            ui.horizontal(|ui| {
                ui.label(pet_status_text);
                ui.label(dead_alive_text);
            });
            let reset_button = ui.add_sized([120., 20.], egui::Button::new("Reset"));
            if reset_button.clicked() {
                *pet = Pet::default();
            };
            if !pet.children.is_empty() {
                ui.heading("Children:");

                for child in &*pet.children {
                    ui.label(child.to_owned());
                }
            }

            ui.allocate_space(ui.available_size());
        });
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(1)
    }

    fn max_size_points(&self) -> egui::Vec2 {
        egui::Vec2::INFINITY
    }

    fn persist_native_window(&self) -> bool {
        true
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn warm_up_enabled(&self) -> bool {
        false
    }

    fn post_rendering(&mut self, _window_size_px: [u32; 2], _frame: &eframe::Frame) {}
}
