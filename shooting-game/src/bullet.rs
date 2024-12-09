use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Bullet {
    x: f32,
    y: f32,
}

#[wasm_bindgen]
impl Bullet {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Bullet {
        Bullet { x, y }
    }

    pub fn move_up(&mut self) {
        self.y -= 5.0; // 弾が上に向かって進む
    }

    pub fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}

