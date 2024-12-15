use wasm_bindgen::prelude::*;

#[derive(Clone, Copy)]
pub struct Position{
    pub x: f32,
    pub y: f32,
}

impl Position{
    pub fn new(x: f32, y: f32) -> Position {
        Position{x, y}
    }
}