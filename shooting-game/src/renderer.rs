use std::sync::{Arc};
use tokio::sync::Mutex;

use anyhow::{anyhow, Result};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use crate::enemy::Enemy;
use crate::player::Player;
use crate::bullet::Bullet;
use crate::logger::Logger;
use web_sys::{console, window, CanvasRenderingContext2d, Document, HtmlCanvasElement, HtmlImageElement};

#[derive(Clone)]
pub struct Renderer {
    pub ctx: CanvasRenderingContext2d,
    pub canvas: HtmlCanvasElement,
    pub images: std::collections::HashMap<String, web_sys::HtmlImageElement>,
}

impl Renderer {
    pub fn new() -> Renderer {
        let window = window().expect("no global `window` exists");
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

        Renderer {
            ctx,
            canvas,
            images: std::collections::HashMap::new(),
        }
    }

    pub fn load_images(&mut self) {
        let image_sources = vec![
            ("player", "images/player.png"),
            ("bullet", "images/bullet.png"),
            ("enemy", "images/enemy.png"),
            ("heart", "images/heart.png"),
            ("background", "images/background.png"),
        ];

        // Arc<Mutex<Renderer>> を作成
        let renderer_arc = Arc::new(Mutex::new(self.clone()));

        for (name, src) in image_sources {
            let renderer_clone = Arc::clone(&renderer_arc); // renderer の clone を作成

            spawn_local(async move {
                match load_image(name, src).await {
                    Ok(image) => {
                        // 画像のロード成功後に Renderer の images に挿入
                        let mut renderer = renderer_clone.lock().await;  // ここで lock() を await
                        renderer.images.insert(name.to_string(), image);
                        console::log_1(&format!("Image '{}' loaded successfully!", name).into());
                        // 画像を使う処理をここで行う
                    }
                    Err(e) => {
                        // 画像ロードに失敗した場合の処理
                        console::log_1(&format!("Error loading image: {:?}", e).into());
                    }
                }
            });
        }
    }

    pub fn clear(&self) {
        self.ctx.clear_rect(0.0, 0.0, self.canvas.width() as f64, self.canvas.height() as f64);
    }

    pub fn draw_background(&self) {
        if let Some(background) = self.images.get("background") {
            self.ctx.draw_image_with_html_image_element(
                background,
                0.0,
                0.0,
            ).unwrap();
        }
    }

    pub fn draw_player(&self, player: &Player) {
        if let Some(player_image) = self.images.get("player") {
            self.ctx.draw_image_with_html_image_element(
                player_image,
                (player.position.x - player.width / 2.0) as f64,
                (player.position.y - player.height / 2.0) as f64,
            ).unwrap();
        }
    }

    pub fn draw_bullets(&self, bullets: &Vec<Bullet>) {
        if let Some(bullet_image) = self.images.get("bullet") {
            for bullet in bullets {
                self.ctx.draw_image_with_html_image_element(
                    bullet_image,
                    (bullet.position.x - bullet.width / 2.0) as f64,
                    (bullet.position.y - bullet.height / 2.0) as f64,
                ).unwrap();
            }
        }
    }

    pub fn draw_enemies(&self, enemies: &Vec<Enemy>) {
        if let Some(enemy_image) = self.images.get("enemy") {
            for enemy in enemies {
                self.ctx.draw_image_with_html_image_element(
                    enemy_image,
                    (enemy.position.x - enemy.width / 2.0) as f64,
                    (enemy.position.y - enemy.height / 2.0) as f64
                ).unwrap();
            }
        }
    }

    pub fn draw_score(&self, score: u32) {
        self.ctx.set_font("20px Arial");
        self.ctx.set_fill_style(&wasm_bindgen::JsValue::from_str("white"));
        self.ctx.fill_text(&format!("Score: {}", score), 20.0, self.canvas.height() as f64 - 20.0).unwrap();
    }

    pub fn draw_life(&self, life: u32) {
        let heart_size = 45.0;
        if let Some(heart_image) = self.images.get("heart") {
            for i in 0..life {
                self.ctx.draw_image_with_html_image_element(
                    heart_image,
                    self.canvas.width() as f64 - 10.0 - (i + 1) as f64 * (heart_size + 5.0),
                    self.canvas.height() as f64 - heart_size - 10.0
                ).unwrap();
            }
        }
    }

    pub fn draw_center_points(&self, player: &Player, bullets: &Vec<Bullet>, enemies: &Vec<Enemy>) {
        // プレイヤーの中心座標
        let player_center_x = player.position.x;
        let player_center_y = player.position.y ;
        self.draw_point(player_center_x as f64, player_center_y as f64, "red");

        // 弾丸の中心座標
        for bullet in bullets {
            let bullet_center_x = bullet.position.x;
            let bullet_center_y = bullet.position.y;
            self.draw_point(bullet_center_x as f64, bullet_center_y as f64, "blue");
        }

        // 敵の中心座標
        for enemy in enemies {
            let enemy_center_x = enemy.position.x;
            let enemy_center_y = enemy.position.y;
            self.draw_point(enemy_center_x as f64, enemy_center_y as f64, "green");
        }
    }

    pub fn draw_point(&self, x: f64, y: f64, color: &str) {
        self.ctx.begin_path();
        self.ctx.arc(x, y, 3.0, 0.0, std::f64::consts::PI * 2.0).unwrap(); // 半径3の円を描画
        self.ctx.set_fill_style(&wasm_bindgen::JsValue::from_str(color));
        self.ctx.fill();
        self.ctx.close_path();
    }
}

async fn load_image(name: &str, src: &str) -> Result<(HtmlImageElement), JsValue>{
    let image = web_sys::HtmlImageElement::new().unwrap();
    wait_for_image_to_load(&image).await?;
    image.set_src(src);
    Ok((image))
}

async fn wait_for_image_to_load(image: &HtmlImageElement) -> Result<(), JsValue> {
    // JavaScript の Promise を作成
    let promise = js_sys::Promise::new(&mut |resolve, reject| {
        // onload イベントの設定
        let resolve = resolve.clone();
        let closure = Closure::wrap(Box::new(move || {
            resolve.call0(&JsValue::NULL).unwrap();
        }) as Box<dyn FnMut()>);

        image.set_onload(Some(closure.as_ref().unchecked_ref()));
        closure.forget();

        // onerror イベントの設定（エラーハンドリング）
        let reject = reject.clone();
        let error_closure = Closure::wrap(Box::new(move || {
            reject.call0(&JsValue::NULL).unwrap();
        }) as Box<dyn FnMut()>);

        image.set_onerror(Some(error_closure.as_ref().unchecked_ref()));
        error_closure.forget();
    });

    // Promise が解決するのを待つ
    JsFuture::from(promise).await?;
    Ok(())
}