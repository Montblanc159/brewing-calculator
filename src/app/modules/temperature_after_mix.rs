use crate::app::modules::math::compute_temperature_after_mix;
use crate::app::modules::ui_defaults::*;
use eframe::*;
use egui::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct TemperatureAfterMix {
    opened: bool,
    temp_a: f32,
    vol_a: f32,
    temp_b: f32,
    vol_b: f32,
    mix_temperature: f32,
}

impl Default for TemperatureAfterMix {
    fn default() -> Self {
        Self {
            opened: false,
            temp_a: 0.0,
            vol_a: 0.0,
            temp_b: 0.0,
            vol_b: 0.0,
            mix_temperature: 0.0,
        }
    }
}

impl TemperatureAfterMix {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        if ui.button("Température après mélange").clicked() {
            self.opened = !self.opened;
        };

        egui::Window::new("Température après mélange")
            .open(&mut self.opened)
            .default_size([250., 250.])
            .show(ui.ctx(), |ui| {
                ui.horizontal(|ui| {
                    ui.label("Température a (°C): ");
                    ui.add(Slider::new(&mut self.temp_a, -30.0..=110.));
                });

                ui.add_space(DEFAULT_SPACING);

                ui.horizontal(|ui| {
                    ui.label("Volume a (L): ");
                    ui.add(Slider::new(&mut self.vol_a, 0.0..=30000.0));
                });

                ui.add_space(DEFAULT_SPACING);

                ui.horizontal(|ui| {
                    ui.label("Température b (°C): ");
                    ui.add(Slider::new(&mut self.temp_b, -30.0..=110.));
                });

                ui.add_space(DEFAULT_SPACING);

                ui.horizontal(|ui| {
                    ui.label("Volume b (L): ");
                    ui.add(Slider::new(&mut self.vol_b, 0.0..=30000.0));
                });

                self.mix_temperature =
                    compute_temperature_after_mix(self.temp_a, self.vol_a, self.temp_b, self.vol_b);

                ui.add_space(DEFAULT_SPACING);

                ui.label(format!(
                    "Température du mélange (°C): {}",
                    self.mix_temperature
                ))
            });
    }
}
