use std::rc::Rc;

use web_sys::HtmlImageElement;

use crate::position::Position;

#[derive(Clone)]
pub struct Enemy {
    pub position: Position,
    velocity: f32,
    pub image: Rc<HtmlImageElement>
}

impl Enemy {
    pub fn new(x: f32, y: f32, image: Rc<HtmlImageElement>) -> Enemy {
        Enemy { position: Position::new(x, y), velocity: 2.0, image: image}
    }

    pub fn move_down(&mut self) {
        self.position.y += self.velocity;
    }

    pub fn get_position(&self) -> Position {
        self.position.clone()
    }
}
