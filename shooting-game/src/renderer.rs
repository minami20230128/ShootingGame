use anyhow::Result;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use crate::{enemy::Enemy, engine};
use crate::player::Player;
use crate::bullet::Bullet;
use crate::logger::Logger;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement, HtmlElement, HtmlImageElement, NodeList};

pub struct Renderer {
    pub ctx: CanvasRenderingContext2d,
    pub canvas: HtmlCanvasElement,
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
        }
    }

    pub fn clear(&self) {
        self.ctx.clear_rect(0.0, 0.0, self.canvas.width() as f64, self.canvas.height() as f64);
    }

    pub fn draw_background(&self, background_image: &HtmlImageElement) {
        self.ctx.draw_image_with_html_image_element(
            background_image,
            0.0,
            0.0,
        ).unwrap();
    }

    pub fn draw_player(&self, player: &Player) {
        self.ctx.draw_image_with_html_image_element(
            &player.image,
            (player.position.x - player.image.width() as f32 / 2.0) as f64,
            (player.position.y - player.image.height() as f32 / 2.0) as f64,
        ).unwrap();
    }

    pub fn draw_bullets(&self, bullets: &Vec<Bullet>) {
        for bullet in bullets {
            self.ctx.draw_image_with_html_image_element(
                &bullet.image,
                (bullet.position.x - bullet.image.width() as f32 / 2.0) as f64,
                (bullet.position.y - bullet.image.height() as f32 / 2.0) as f64,
            ).unwrap();
        }
    }

    pub fn draw_enemies(&self, enemies: &Vec<Enemy>) {
        for enemy in enemies {
            self.ctx.draw_image_with_html_image_element(
                &enemy.image,
                (enemy.position.x - enemy.image.width() as f32 / 2.0) as f64,
                (enemy.position.y - enemy.image.height() as f32 / 2.0) as f64
            ).unwrap();
        }
    }

    pub fn draw_score(&self, score: u32) {
        self.ctx.set_font("20px Arial");
        self.ctx.set_fill_style(&wasm_bindgen::JsValue::from_str("white"));
        self.ctx.fill_text(&format!("Score: {}", score), 20.0, self.canvas.height() as f64 - 20.0).unwrap();
    }

    pub fn draw_life(&self, life: u32, heart_image: &HtmlImageElement) {
        let heart_size = 45.0;
        for i in 0..life {
            self.ctx.draw_image_with_html_image_element(
                heart_image,
                self.canvas.width() as f64 - 10.0 - (i + 1) as f64 * (heart_size + 5.0),
                self.canvas.height() as f64 - heart_size - 10.0
            ).unwrap();
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

    pub fn put_button(inner_html : &str) -> Result<(), JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let button = document.create_element("button")?.dyn_into::<HtmlElement>()?;
        button.set_inner_html(inner_html);
        button
        .set_attribute(
            "style",
            "position: absolute; top: 300px; left: 400px; z-index: 10;",
        )
        .unwrap();

        let closure = Closure::wrap(Box::new(move || {
            engine::start_game();
            Renderer::remove_all_buttons();
            web_sys::console::log_1(&"Button clicked!".into());
        }) as Box<dyn Fn()>);

        button.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget(); // ClosureをJavaScriptで保持させる
    
        // <body>にボタンを追加
        document.body().unwrap().append_child(&button)?;
    
        Ok(())
    }

    pub fn remove_all_buttons() -> Result<(), JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();

        // ページ内のすべてのボタン要素を取得
        let buttons: NodeList = document.query_selector_all("button").unwrap();

        // ボタン要素を削除する
        for i in (0..buttons.length()).rev() {
            if let Some(button) = buttons.get(i) {
                if let Some(button_element) = button.dyn_ref::<HtmlElement>() {
                    button_element.remove();
                }
            }
        }

        Ok(())
    }
}
