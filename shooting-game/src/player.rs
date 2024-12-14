use wasm_bindgen::prelude::*;
use js_sys::Array;
use image::GenericImageView;
use crate::position::Position;

#[wasm_bindgen]
pub struct Player {
    position: Position,
    width: f32,
    height: f32,
    life: u32,
}


#[wasm_bindgen]
impl Player {
    // コンストラクタ相当の関数
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Player {
        Player { position: Position::new(x, y), width, height, life: 3, }
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
            self.position.x -= 30.0; // 左に移動
        }
    }

    // 右に移動するメソッド
    pub fn move_right(&mut self, canvas_width: f32) {
        if self.position.x < canvas_width - self.width / 2.0 {
            self.position.x += 30.0; // 右に移動
        }
    }
}