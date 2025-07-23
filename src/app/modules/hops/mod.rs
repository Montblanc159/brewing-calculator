mod boil;
mod whirlpool;

use crate::app::modules::ui_defaults::*;
use eframe::*;
use egui::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct Hops {
    boil: boil::BoilHops,
    whirlpool: whirlpool::WhirlpoolHops,
    pub ibu: f32,
    pub original_gravity: f32,
    pub batch_size: u16,
}

impl super::AppModule for Hops {
    fn new() -> Self {
        Self {
            boil: boil::BoilHops::new(),
            whirlpool: whirlpool::WhirlpoolHops::new(),
            ..Default::default()
        }
    }

    fn show(&mut self, ui: &mut Ui) {
        self.whirlpool.batch_size = self.batch_size;
        self.whirlpool.original_gravity = self.original_gravity;

        self.whirlpool.show(ui);

        ui.add_space(DEFAULT_SPACING);

        self.boil.target_ibu = self.ibu - self.whirlpool.total_ibu;
        self.boil.batch_size = self.batch_size;
        self.boil.original_gravity = self.original_gravity;

        self.boil.show(ui);
    }
}
