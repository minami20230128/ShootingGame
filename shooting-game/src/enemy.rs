use wasm_bindgen::prelude::*;
use crate::position::Position;

#[wasm_bindgen]
pub struct Enemy {
    position: Position,
    speed: f32,
}

#[wasm_bindgen]
impl Enemy {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Enemy {
        Enemy { position: Position::new(x, y), speed: 2.0 }
    }

    pub fn move_down(&mut self) {
        self.position.y += self.speed;
    }

    pub fn get_position(&self) -> Position {
        self.position.clone()
    }
}
