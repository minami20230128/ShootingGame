use crate::position::Position;

pub struct Player {
    pub position: Position,
    pub width: f32,
    pub height: f32,
    life: u32,
    velocity: u32,
}

impl Player {
    // コンストラクタ相当の関数
    pub fn new(x: f32, y: f32 ) -> Player {
        Player { position: Position::new(x, y), width: 180.0, height: 180.0, life: 3, velocity: 30}
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
        if self.position.x > self.width / 2.0 {
            self.position.x -= self.velocity as f32; // 左に移動
        }
    }

    // 右に移動するメソッド
    pub fn move_right(&mut self, canvas_width: f32) {
        if self.position.x < canvas_width - self.width / 2.0 {
            self.position.x += self.velocity as f32; // 右に移動
        }
    }
}