mod player;
mod bullet;
use wasm_bindgen::prelude::*;

// `player` モジュールと `bullet` モジュールから公開された構造体をエクスポート
pub use player::Player;
pub use bullet::Bullet;

#[wasm_bindgen(start)]
pub fn run() {
    

}