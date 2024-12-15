use wasm_bindgen::prelude::*;
use crate::enemy::Enemy;
use crate::player::Player;
use crate::bullet::Bullet;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub struct Renderer {
    ctx: CanvasRenderingContext2d,
    canvas: HtmlCanvasElement,
    images: std::collections::HashMap<String, web_sys::HtmlImageElement>,
}

impl Renderer {
    pub fn new(ctx: CanvasRenderingContext2d) -> Renderer {
        let canvas = ctx.canvas().unwrap().dyn_into::<HtmlCanvasElement>().unwrap();
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

        for (name, src) in image_sources {
            let image = web_sys::HtmlImageElement::new().unwrap();
            image.set_src(src);
            self.images.insert(name.to_string(), image);
        }
    }

    pub fn clear(&self) {
        self.ctx.clear_rect(0.0, 0.0, self.canvas.width() as f64, self.canvas.height() as f64);
    }

    pub fn draw_background(&self) {
        if let Some(background) = self.images.get("background") {
            self.ctx.draw_image_with_html_image_element(
                background,
                self.canvas.width() as f64,
                self.canvas.height() as f64,
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
                    (bullet.position.x - 10.0) as f64,
                    (bullet.position.y - 10.0) as f64,
                ).unwrap();
            }
        }
    }

    pub fn draw_enemies(&self, enemies: &Vec<Enemy>) {
        if let Some(enemy_image) = self.images.get("enemy") {
            for enemy in enemies {
                self.ctx.draw_image_with_html_image_element(
                    enemy_image,
                    (enemy.position.x - 20.0) as f64,
                    (enemy.position.y - 20.0) as f64
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
        let heart_size = 30.0;
        if let Some(heart_image) = self.images.get("heart") {
            for i in 0..life {
                self.ctx.draw_image_with_html_image_element(
                    heart_image,
                    self.canvas.width() as f64 - 30.0 - (i + 1) as f64 * (heart_size + 5.0),
                    self.canvas.height() as f64 - heart_size - 10.0
                ).unwrap();
            }
        }
    }
}