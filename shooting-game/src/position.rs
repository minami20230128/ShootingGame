use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Position{
    pub x: f32,
    pub y: f32,
}

#[wasm_bindgen]
impl Position{
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Position {
        Position{x, y}
    }
}