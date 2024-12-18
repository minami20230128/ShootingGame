use crate::position::Position;

#[derive(Clone, Copy)]
pub struct Enemy {
    pub position: Position,
    pub width: f32,
    pub height:f32,
    pub speed: f32,
}

impl Enemy {
    pub fn new(x: f32, y: f32) -> Enemy {
        Enemy { position: Position::new(x, y), width: 216.0, height: 216.0, speed: 2.0 }
    }

    pub fn move_down(&mut self) {
        self.position.y += self.speed;
    }

    pub fn get_position(&self) -> Position {
        self.position.clone()
    }
}
