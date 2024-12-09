use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Player {
    x: f32,
    y: f32,
}

#[wasm_bindgen]
impl Player {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Player {
        Player { x, y }
    }

    pub fn move_left(&mut self) {
        self.x -= 10.0;
    }

    pub fn move_right(&mut self) {
        self.x += 10.0;
    }

    pub fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}

