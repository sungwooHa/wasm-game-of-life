extern crate wasm_bindgen;

use universe::Universe;
use wasm_bindgen::prelude::*;
mod cell;
pub mod universe;
mod utils;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-game-of-life!");
}
