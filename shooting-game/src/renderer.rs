use anyhow::{anyhow, Result};
use crate::enemy::Enemy;
use crate::player::Player;
use crate::bullet::Bullet;
use crate::logger::Logger;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Window};

pub struct Renderer {
    pub ctx: CanvasRenderingContext2d,
    images: std::collections::HashMap<String, web_sys::HtmlImageElement>,
}

impl Renderer {
    pub fn new(ctx: CanvasRenderingContext2d,) -> Renderer {
        Renderer {
            ctx,
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

        for (name, src) in image_sources {
            let image = web_sys::HtmlImageElement::new().unwrap();
            image.set_src(src);
            self.images.insert(name.to_string(), image);
        }
    }

    pub fn clear(&self, canvas: &HtmlCanvasElement) {
        self.ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    }

    pub fn draw_background(&self, canvas: &HtmlCanvasElement) {
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

    pub fn draw_score(&self, score: u32, canvas: &HtmlCanvasElement) {
        self.ctx.set_font("20px Arial");
        self.ctx.set_fill_style(&wasm_bindgen::JsValue::from_str("white"));
        self.ctx.fill_text(&format!("Score: {}", score), 20.0, canvas.height() as f64 - 20.0).unwrap();
    }

    pub fn draw_life(&self, life: u32, canvas: &HtmlCanvasElement) {
        let heart_size = 45.0;
        if let Some(heart_image) = self.images.get("heart") {
            for i in 0..life {
                self.ctx.draw_image_with_html_image_element(
                    heart_image,
                    canvas.width() as f64 - 10.0 - (i + 1) as f64 * (heart_size + 5.0),
                    canvas.height() as f64 - heart_size - 10.0
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
