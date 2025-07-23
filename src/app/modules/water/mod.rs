use crate::app::modules::math;
use crate::app::modules::ui_defaults::*;
use eframe::*;
use egui::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct Water {
    pub batch_size: u16,
    pub grain_weight: f32,
    mash_water_ratio: f32,
    evaporation_rate: f32,
    mash_water_vol: f32,
    post_mash_water_vol: f32,
    sparge_water_vol: f32,
    pre_ebullition_water_vol: f32,
}

impl super::AppModule for Water {
    fn new() -> Self {
        Self {
            mash_water_ratio: 3.5,
            evaporation_rate: 10.,
            ..Default::default()
        }
    }

    fn show(&mut self, ui: &mut Ui) {
        ui.heading("Eau");
        ui.add_space(DEFAULT_SPACING);

        egui::Frame::new()
            .fill(LIGHTER_COLOR)
            .inner_margin(DEFAULT_PADDING)
            .corner_radius(DEFAULT_CORNER_RADIUS)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Ratio d'eau à l'empâtage (L/kg): ");
                    ui.add(Slider::new(&mut self.mash_water_ratio, 0.0..=10.0));
                });

                ui.add_space(DEFAULT_SPACING);

                ui.horizontal(|ui| {
                    ui.label("Taux d'évaporation (%): ");
                    ui.add(Slider::new(&mut self.evaporation_rate, 0.0..=20.0));
                });

                ui.add_space(DEFAULT_SPACING);

                ui.label(format!(
                    "Volume d'eau à l'empatage (L): {:.3}",
                    self.mash_water_vol
                ));

                ui.add_space(DEFAULT_SPACING);

                ui.label(format!(
                    "Volume d'eau de rinçage (L): {:.3}",
                    self.sparge_water_vol
                ));

                ui.add_space(DEFAULT_SPACING);

                ui.label(format!(
                    "Volume d'eau pré-ébullition (L): {:.3}",
                    self.pre_ebullition_water_vol
                ));

                self.mash_water_vol =
                    math::compute_mash_water_vol(self.grain_weight, self.mash_water_ratio);

                self.post_mash_water_vol =
                    math::compute_post_mash_water_vol(self.mash_water_vol, self.grain_weight);

                self.sparge_water_vol = math::compute_sparge_water_vol(
                    self.batch_size,
                    self.evaporation_rate,
                    self.post_mash_water_vol,
                );

                self.pre_ebullition_water_vol = math::compute_pre_ebullition_water_vol(
                    self.sparge_water_vol,
                    self.post_mash_water_vol,
                );
            });
    }
}
