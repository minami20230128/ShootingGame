use wasm_bindgen::prelude::*;
use crate::position::Position;

pub struct Bullet {
    pub position: Position,
    pub width: f32,
    pub height: f32,
}

impl Bullet {
    pub fn new(x: f32, y: f32) -> Bullet {
        Bullet { position: Position::new(x, y), width: 90.0, height: 90.0}
    }

    pub fn move_up(&mut self) {
        self.position.y -= 5.0; // 弾が上に向かって進む
    }

       // 配列として位置情報を返す
    pub fn get_position(&self) -> Position {
        self.position.clone()
    }
}

