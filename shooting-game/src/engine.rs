use crate::game::Game;
use std::cell::RefCell;
use std::rc::Rc;
use anyhow::{anyhow, Result};
use web_sys::window;
use wasm_bindgen::{
    prelude::Closure, JsCast, JsValue,
};

pub struct GameLoop{}
// グローバルなゲームインスタンス
static mut GAME: Option<Rc<RefCell<Game>>> = None;

pub fn start_game() {
    let window = window().expect("no global window exists");
    // ゲームの初期化
    let game_result = Game::new("gameCanvas");

    // Gameインスタンス生成のエラーハンドリング
    let mut game = match game_result {
        Ok(game) => Rc::new(RefCell::new(game)),
        Err(err) => {
            // JavaScriptコンソールにエラーメッセージを表示
            web_sys::console::error_1(&err);
            return;
        }
    };

    // グローバルなゲームインスタンスを設定
    unsafe {
        GAME = Some(game.clone());
    }

    // キーボードイベントリスナーの設定
    {
        let game_rc = game.clone();
        let key_down_closure =
            Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                let key = event.key();
                game_rc.borrow_mut().key_down(key);
            }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
        window
            .add_event_listener_with_callback(
                "keydown",
                key_down_closure.as_ref().unchecked_ref(),
            )
            .expect("failed to add keydown listener");
        key_down_closure.forget();
    }

    {
        let game_rc = game.clone();
        let key_up_closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            let key = event.key();
            game_rc.borrow_mut().key_up(key);
        }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
        window
            .add_event_listener_with_callback("keyup", key_up_closure.as_ref().unchecked_ref())
            .expect("failed to add keyup listener");
        key_up_closure.forget();
    }

    game.borrow_mut().renderer.load_images();
    //Game::setup_event_listeners(game);
    Game::start(game);

}