use std::rc::Rc;

use web_sys::HtmlImageElement;

use crate::position::Position;

pub struct Bullet {
    pub position: Position,
    velocity: u32,
    pub image: Rc<HtmlImageElement>
}

impl Bullet {
    pub fn new(x: f32, y: f32, image: Rc<HtmlImageElement>) -> Bullet {
        Bullet { position: Position::new(x, y), velocity: 5, image: image}
    }

    pub fn move_up(&mut self) {
        self.position.y -= self.velocity as f32; // 弾が上に向かって進む
    }

       // 配列として位置情報を返す
    pub fn get_position(&self) -> Position {
        self.position.clone()
    }
}

