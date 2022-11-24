/// We derive Deserialize/Serialize so we can persist app state on shutdown.
// if we add new fields, give them default values when deserializing old state
use egui::{Color32, RichText};

use crate::Pet;
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
#[derive(Default)]
pub struct PetApp {
    // this how you opt-out of serialization of a member
    #[serde(skip)]
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
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { pet } = self;

        let pet_status = if pet.is_alive() {
            RichText::new("Alive").color(Color32::GREEN)
        } else {
            RichText::new("Dead").color(Color32::RED)
        };

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
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
                ui.set_max_width(175.0);

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
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.label(pet.status());
                if ui.button("Walk").clicked() {
                    pet.walk();
                }
                if ui.button("Feed").clicked() {
                    pet.feed()
                };
                if ui.button("Grow Up").clicked() {
                    pet.grow_up()
                };
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(format!("{} is:", pet.name));
                ui.label(pet_status);
                if ui.button("Reset").clicked() {
                    *pet = Pet::default()
                };
            });
        });
    }
}
