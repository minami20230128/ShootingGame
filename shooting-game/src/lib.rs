mod player;
mod bullet;
mod enemy;
mod position;
use wasm_bindgen::prelude::*;

// `player` モジュールと `bullet` モジュールから公開された構造体をエクスポート
pub use player::Player;
pub use bullet::Bullet;
pub use enemy::Enemy;
pub use position::Position;

#[wasm_bindgen(start)]
pub fn run() {
    

}