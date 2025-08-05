use egui::Ui;
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
use std::fs;
use std::{marker, process::exit};

pub mod base;
pub mod bjcp_style_index;
pub mod equilibrium_pressure;
pub mod fermentecibles;
pub mod hops;
pub mod ingredients_index;
pub mod math;
pub mod temperature_after_mix;
pub mod ui_defaults;
pub mod water;
pub mod yeast;

pub trait AppModule {
    fn new() -> Self;
    fn show(&mut self, ui: &mut Ui) -> ();
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Default)]
pub struct JsonParser<T> {
    path: &'static str,
    _owns_t: marker::PhantomData<T>,
}

#[cfg(not(target_arch = "wasm32"))]
impl<T> JsonParser<T>
where
    T: for<'a> Deserialize<'a> + Serialize + Default,
{
    pub fn new(path: &'static str) -> Self {
        Self {
            path,
            ..Default::default()
        }
    }

    pub fn parse_json(&self) -> Vec<T> {
        // Use a `match` block to return the
        // file `contents` as a `Data struct: Ok(d)`
        // or handle any `errors: Err(_)`.
        let data = match serde_json::from_str(&self.fetch_json()) {
            // If successful, return data as `Data` struct.
            // `d` is a local variable.
            Ok(d) => d,
            // Handle the `error` case.
            Err(e) => {
                // Write `msg` to `stderr`.
                eprintln!("Unable to load data from {0}: {e}", self.path);
                // Exit the program with exit code `1`.
                exit(1);
            }
        };

        data
    }

    fn fetch_json(&self) -> String {
        match fs::read_to_string(self.path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Could not read file `{0}`: {e}", self.path);
                exit(1);
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[derive(Default)]
pub struct WasmJsonParser<T> {
    file: String,
    _owns_t: marker::PhantomData<T>,
}

#[cfg(target_arch = "wasm32")]
// When compiling to web using trunk:
impl<T> WasmJsonParser<T>
where
    T: for<'a> Deserialize<'a> + Serialize + Default,
{
    pub fn new(file: String) -> Self {
        Self {
            file,
            ..Default::default()
        }
    }

    pub fn parse_json(&self) -> Vec<T> {
        // Use a `match` block to return the
        // file `contents` as a `Data struct: Ok(d)`
        // or handle any `errors: Err(_)`.
        let data = match serde_json::from_str(&self.file) {
            // If successful, return data as `Data` struct.
            // `d` is a local variable.
            Ok(d) => d,
            // Handle the `error` case.
            Err(e) => {
                // Write `msg` to `stderr`.
                eprintln!("Unable to load data from {0}: {e}", self.file);
                // Exit the program with exit code `1`.
                exit(1);
            }
        };

        data
    }
}
