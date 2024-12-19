use crate::game::Game;
use crate::position::Position;
use std::cell::RefCell;
use std::rc::Rc;
use anyhow::{anyhow, Result};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent, window};
use wasm_bindgen::{
    prelude::Closure, JsCast, JsValue,
};
use crate::logger::Logger;

pub struct GameLoop{}
// グローバルなゲームインスタンス
static mut GAME: Option<Rc<RefCell<Game>>> = None;

pub fn start_game() {
    Logger::log("start_game");
    unsafe {
        if GAME.is_some() {
            web_sys::console::log_1(&"Game is already initialized".into());
            return;
        }
    }
    let window = window().expect("no global window exists");

    let document = window.document().unwrap();

    let canvas: HtmlCanvasElement = document
        .get_element_by_id("gameCanvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .expect("gameCanvas should be a HtmlCanvasElement");

    let ctx = canvas
        .get_context("2d")
        .expect("should have 2d context")
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .expect("gameCanvas should be a HtmlCanvasElement");

    // ゲームの初期化
    let game_result = Game::new(canvas, ctx);

    // Gameインスタンス生成のエラーハンドリング
    let game = match game_result {
        Ok(game) => Rc::new(RefCell::new(game)),
        Err(err) => {
            // JavaScriptコンソールにエラーメッセージを表示
            web_sys::console::error_1(&err);
            return;
        }
    };

    //グローバルなゲームインスタンスを設定
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
//
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
//
    game.borrow_mut().renderer.load_images();
    Game::start(game.clone());
}