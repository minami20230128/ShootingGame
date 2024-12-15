use wasm_bindgen::prelude::*;
use wasm_bindgen::{
    closure::WasmClosure, closure::WasmClosureFnOnce, prelude::Closure, JsCast, JsValue,
};
use wasm_bindgen_futures;
use anyhow::{anyhow, Result};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window, Document};
use std::rc::Rc;
use std::cell::RefCell;
use crate::enemy::Enemy;
use crate::player::Player;
use crate::bullet::Bullet;
use crate::renderer::Renderer;

pub struct Game {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    renderer: Renderer,
    player: Player,
    bullets: Vec<Bullet>,
    enemies: Vec<Enemy>,
    score: u32,
    running: Rc<RefCell<bool>>,
}

impl Game {
    pub fn new(canvas_id: &str) -> Result<Game, JsValue> {
        let document = window().unwrap().document().unwrap();
        let canvas = document
            .get_element_by_id(canvas_id)
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()?;
        let ctx = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        Ok(Game {
            canvas,
            ctx: ctx.clone(),
            renderer: Renderer::new(ctx),
            player: Player::new(400.0, 500.0, 180.0, 180.0),
            bullets: Vec::new(),
            enemies: Vec::new(),
            score: 0,
            running: Rc::new(RefCell::new(true)),
        })
    }

    pub fn init(&mut self) -> Result<(), JsValue> {
        self.renderer.load_images();
        self.spawn_enemies();
        self.start_game_loop();
        Ok(())
    }

    fn spawn_enemies(&self) {
        let enemies = self.enemies.clone();
        let canvas_width = self.canvas.width() as f64;
        let running = self.running.clone();

        wasm_bindgen_futures::spawn_local(async move {
            loop {
                if !*running.borrow() {
                    break;
                }

                let x = js_sys::Math::random() * canvas_width;
                enemies.push(Enemy::new(x, 0.0));
                wasm_bindgen_futures::JsFuture::from(js_sys::Promise::new(&mut |resolve, _| {
                    let window = window().unwrap();
                    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 2000);
                })).await.unwrap();
            }
        });
    }

    fn start_game_loop(&self) {
        let running = self.running.clone();
        let game = Rc::new(RefCell::new(self.clone()));

        type SharedLoopClosure = Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>>;
        let f: SharedLoopClosure = Rc::new(RefCell::new(None));
        let g = f.clone();

        *g.borrow_mut() = Some(Closure::wrap(Box::new({
            let running = running.clone();
            let game = game.clone();

            move |timestamp: f64| {
                if !*running.borrow() {
                    return;
                }

                game.borrow_mut().update();
                game.borrow_mut().render();

                if let Some(ref closure) = f.borrow().as_ref() {
                    window()
                        .unwrap()
                        .request_animation_frame(closure.as_ref().unchecked_ref())
                        .unwrap();
                }
            }
        }) as Box<dyn FnMut(f64)>));


        window()
            .unwrap()
            .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    }

    fn update(&mut self) {
        for bullet in &mut self.bullets {
            bullet.move_up();
        }

        for enemy in &mut self.enemies {
            enemy.move_down();
        }

        self.check_collisions();
    }

    fn check_collisions(&mut self) {
        // Implement collision logic
    }

    fn render(&self) {
        self.renderer.clear();
        self.renderer.draw_background();
        self.renderer.draw_player(&self.player);
        self.renderer.draw_bullets(&self.bullets);
        self.renderer.draw_enemies(&self.enemies);
        self.renderer.draw_score(self.score);
    }

    pub fn game_over(&self) {
        *self.running.borrow_mut() = false;
        web_sys::window()
            .unwrap()
            .alert_with_message("Game Over!")
            .unwrap();
    }
}