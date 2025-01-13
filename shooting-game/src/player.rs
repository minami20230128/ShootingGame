use std::rc::Rc;
use web_sys::HtmlImageElement;
use crate::position::Position;

pub struct Player {
    pub position: Position,
    life: u32,
    velocity: u32,
    pub image: Rc<HtmlImageElement>
}

impl Player {
    // コンストラクタ相当の関数
    pub fn new(x: f32, y: f32, image: Rc<HtmlImageElement> ) -> Player {
        Player { position: Position::new(x, y), life: 3, velocity: 30, image}
    }

    pub fn decrease_life(&mut self) {
        if self.life > 0 {
            self.life -= 1;
        }
    }

    // プレイヤーのライフを取得
    pub fn get_life(&self) -> u32 {
        self.life
    }

    // プレイヤーの現在の位置を返すメソッド
    pub fn get_position(&self) -> Position {
        self.position.clone()
    }

    // 左に移動するメソッド
    pub fn move_left(&mut self, canvas_width: f32) {
        if self.position.x > self.image.width() as f32 / 2.0 {
            self.position.x -= self.velocity as f32; // 左に移動
        }
    }

    // 右に移動するメソッド
    pub fn move_right(&mut self, canvas_width: f32) {
        if self.position.x < canvas_width - self.image.width() as f32 / 2.0 {
            self.position.x += self.velocity as f32; // 右に移動
        }
    }
}