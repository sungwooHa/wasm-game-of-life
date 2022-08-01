use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)] //each cell is represented as a single byte.
#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}
