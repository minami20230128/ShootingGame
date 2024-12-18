mod player;
mod bullet;
mod enemy;
mod position;
mod renderer;
mod game;
mod logger;
mod game_state;
mod engine;
use wasm_bindgen::prelude::*;
use crate::game::Game;
use crate::logger::Logger;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    Logger::log("start");
    // Gameインスタンスの生成を試みる
    let game_result = Game::new("gameCanvas");
    Logger::log("got canvas");
    // Gameインスタンス生成のエラーハンドリング
    let mut game = match game_result {
        Ok(game) => game,
        Err(err) => {
            // JavaScriptコンソールにエラーメッセージを表示
            web_sys::console::error_1(&err);
            return Err(err);
        }
    };
    Logger::log("game instance initialized");

    // ゲームを初期化し、そのエラーハンドリングを行う
    engine::start_game();

    Ok(())
}