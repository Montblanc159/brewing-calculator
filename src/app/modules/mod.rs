use egui::Ui;

pub mod base;
pub mod bjcp_style_index;
pub mod equilibrium_pressure;
pub mod fermentecibles;
pub mod math;
pub mod temperature_after_mix;
pub mod ui_defaults;
pub mod yeast;

pub trait AppModule {
    fn new() -> Self;
    fn show(&mut self, ui: &mut Ui) -> ();
}
