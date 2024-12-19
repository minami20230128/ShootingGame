mod player;
mod bullet;
mod enemy;
mod position;
mod renderer;
mod game;
mod logger;
mod game_state;
mod engine;
mod enemy_type;
use wasm_bindgen::prelude::*;
use crate::logger::Logger;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    // ゲームを初期化し、そのエラーハンドリングを行う
    engine::start_game();

    Ok(())
}