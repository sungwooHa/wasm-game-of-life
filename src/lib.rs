extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
mod universe;
mod cell;
mod utils;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-game-of-life!");
}
