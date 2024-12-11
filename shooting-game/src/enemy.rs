use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Enemy {
    x: f64,
    y: f64,
    speed: f64,
}

#[wasm_bindgen]
impl Enemy {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Enemy {
        Enemy { x, y, speed: 2.0 }
    }

    pub fn move_down(&mut self) {
        self.y += self.speed;
    }

    pub fn get_position(&self) -> Vec<f64> {
        vec![self.x, self.y]
    }
}
