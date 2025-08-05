mod hops_index;
mod malts_index;
mod yeasts_index;

use eframe::*;
use egui::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct IngredientsIndex {
    hops: hops_index::HopsIndex,
    malts: malts_index::MaltsIndex,
    yeasts: yeasts_index::YeastsIndex,
}

impl super::AppModule for IngredientsIndex {
    fn new() -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let hops = super::JsonParser::<hops_index::HopIng>::new(hops_index::JSON_PATH).parse_json();

        #[cfg(target_arch = "wasm32")]
        let hops =
            super::WasmJsonParser::<hops_index::HopIng>::new(hops_index::HopsIndex::parse_file())
                .parse_json();

        #[cfg(not(target_arch = "wasm32"))]
        let malts =
            super::JsonParser::<malts_index::MaltIng>::new(malts_index::JSON_PATH).parse_json();

        #[cfg(target_arch = "wasm32")]
        let malts = super::WasmJsonParser::<malts_index::MaltIng>::new(
            malts_index::MaltsIndex::parse_file(),
        )
        .parse_json();

        #[cfg(not(target_arch = "wasm32"))]
        let yeasts =
            super::JsonParser::<yeasts_index::YeastIng>::new(yeasts_index::JSON_PATH).parse_json();

        #[cfg(target_arch = "wasm32")]
        let yeasts = super::WasmJsonParser::<yeasts_index::YeastIng>::new(
            yeasts_index::YeastsIndex::parse_file(),
        )
        .parse_json();

        Self {
            hops: hops_index::HopsIndex::new(hops),
            malts: malts_index::MaltsIndex::new(malts),
            yeasts: yeasts_index::YeastsIndex::new(yeasts),
        }
    }

    fn show(&mut self, ui: &mut Ui) {
        self.hops.show(ui);
        self.malts.show(ui);
        self.yeasts.show(ui);
    }
}
