use crate::app::modules::math;
use crate::app::modules::ui_defaults::*;
use eframe::*;
use egui::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct Base {
    name: String,
    style: String,
    bugu: f32,
    pub ebc: u8,
    pub abv: f32,
    pub ibu: f32,
    pub original_gravity: f32,
    pub final_gravity: f32,
    pub efficiency: u8,
    pub batch_size: u16,
}

impl super::AppModule for Base {
    fn new() -> Self {
        Self {
            ibu: 20.,
            original_gravity: 12.,
            efficiency: 75,
            batch_size: 30,
            ..Default::default()
        }
    }

    fn show(&mut self, ui: &mut Ui) {
        self.abv = math::compute_abv(self.original_gravity, self.final_gravity);

        ui.horizontal(|ui| {
            ui.label("Nom : ");
            ui.text_edit_singleline(&mut self.name);
        });

        ui.add_space(DEFAULT_SPACING);

        ui.horizontal(|ui| {
            ui.label("Style : ");
            ui.text_edit_singleline(&mut self.style);
        });

        ui.add_space(DEFAULT_SPACING);

        ui.label(format!("Alcool (%) : {:.1}", self.abv));

        ui.add_space(DEFAULT_SPACING);

        ui.label(format!(
            "EBC : {} ({:.1} SRM)",
            self.ebc,
            math::convert_ebc_to_srm(self.ebc)
        ));

        ui.add_space(DEFAULT_SPACING);

        ui.horizontal(|ui| {
            ui.label("IBU : ");
            ui.add(Slider::new(&mut self.ibu, 0.0..=150.0));
        });

        ui.add_space(DEFAULT_SPACING);

        ui.label(format!("BUGU : {:.2}", self.bugu));

        self.bugu = math::compute_bugu(self.ibu, self.original_gravity);

        ui.add_space(DEFAULT_SPACING);

        ui.horizontal(|ui| {
            ui.label("Densité initiale (°P) : ");
            ui.add(Slider::new(&mut self.original_gravity, 0.0..=25.0));
            ui.label(format!(
                "{:.3} SG",
                math::convert_plato_to_sg(self.original_gravity)
            ));
        });

        ui.add_space(DEFAULT_SPACING);

        ui.label(format!(
            "Densité finale (°P): {:.1} ({:.3} SG)",
            self.final_gravity,
            math::convert_plato_to_sg(self.final_gravity)
        ));

        ui.add_space(DEFAULT_SPACING);

        ui.horizontal(|ui| {
            ui.label("Efficacité (%): ");
            ui.add(Slider::new(&mut self.efficiency, 0..=100));
        });

        ui.add_space(DEFAULT_SPACING);

        ui.horizontal(|ui| {
            ui.label("Volume (L): ");
            ui.add(Slider::new(&mut self.batch_size, 0..=30000));
        });
    }
}
