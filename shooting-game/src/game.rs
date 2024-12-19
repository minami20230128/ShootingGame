use wasm_bindgen::{
    prelude::Closure, JsCast, JsValue,
};
use wasm_bindgen_futures;
use anyhow::{anyhow, Result};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent, window};
use std::rc::Rc;
use std::cell::RefCell;
use crate::enemy::Enemy;
use crate::player::Player;
use crate::bullet::Bullet;
use crate::renderer::{self, Renderer};
use crate::logger::Logger;
use crate::game_state::GameState;
use crate::enemy_type::EnemyType;
use crate::enemy_type::EnemySpawnInfo;

pub struct Game {
    pub renderer: Renderer,
    player: Player,
    bullets: Vec<Bullet>,
    enemies: Rc<RefCell<Vec<Enemy>>>,
    score: u32,
    state : GameState,
    keys_pressed: Vec<String>,
    last_enemy_spawn: f64,
    enemies_spawn_info: Vec<EnemySpawnInfo>,
}

impl Game {
    pub fn new() -> Result<Game, JsValue> {
        Logger::log("Game new");

        Ok(Game {
            renderer: Renderer::new(),
            player: Player::new(400.0, 500.0),
            bullets: Vec::new(),
            enemies: Rc::new(RefCell::new(Vec::new())),
            score: 0,
            state: GameState::Playing,
            keys_pressed: Vec::new(),
            last_enemy_spawn: 0.0,
            enemies_spawn_info: vec![
                EnemySpawnInfo {
                    enemy_type: EnemyType::Regular,
                    spawn_interval: 2000.0, // 2秒間隔
                    last_spawn_time: 0.0,
                }
            ],
        })
    }

    pub fn start(game_rc: Rc<RefCell<Self>>) {
        let closure = Closure::wrap(Box::new(move |timestamp: f64| {
            let mut game = game_rc.borrow_mut();
            if game.state == GameState::Playing {
                game.process_frame(timestamp);

                // 再度アニメーションフレームを要求
                Game::start(game_rc.clone());
            } else if game.state == GameState::GameOver {
                game.renderer.draw_life(game.player.get_life());
                game.game_over();
            }
        }) as Box<dyn FnMut(f64)>);

        web_sys::window()
            .unwrap()
            .request_animation_frame(closure.as_ref().unchecked_ref())
            .unwrap();

        closure.forget(); // クロージャをメモリに保持させる
    }

    fn process_frame(&mut self, current_time: f64) {
        if self.last_enemy_spawn == 0.0 {
            self.last_enemy_spawn = current_time;
        }

        let regular_enemy_info = self.enemies_spawn_info.get(0).unwrap();
        if current_time - regular_enemy_info.last_spawn_time > regular_enemy_info.spawn_interval {
            self.spawn_enemy(EnemyType::Regular);
            if let Some(spawn_info) = self.enemies_spawn_info.get_mut(0) {
                spawn_info.last_spawn_time = current_time;
            }
        }

        self.update();
        self.render();
    }

    fn spawn_enemy(&mut self, enemy_type: EnemyType) {
        let mut enemies = self.enemies.clone();
        let canvas_width = self.renderer.canvas.width() as f64;
        let state = self.state.clone();
    
        if state != GameState::Playing {
            return;
        }

        let x = (js_sys::Math::random() * canvas_width) as f32;

        match enemy_type {
            EnemyType::Regular => {
                enemies.borrow_mut().push(Enemy::new(x, 0.0));
            }
            EnemyType::Fast => {
                //enemies.borrow_mut().push(Enemy::new(x, 0.0));
            }
            EnemyType::Strong => {
                //enemies.borrow_mut().push(Enemy::new(x, 0.0));
            }
        }
    }

    fn update(&mut self) {
        for bullet in &mut self.bullets {
            bullet.move_up();
        }

        for enemy in self.enemies.borrow_mut().iter_mut() {
            enemy.move_down();
        }

        self.check_collisions();
    }

    // 衝突判定をチェックするメイン関数
    fn check_collisions(&mut self) {
        //Logger::log(&format!("life: {}", self.player.get_life()));
        
        // 弾と敵の衝突判定
        self.check_bullet_enemy_collisions();

        // プレイヤーと敵の衝突判定
        self.check_player_enemy_collisions();
    }

    // 弾と敵の衝突判定
    fn check_bullet_enemy_collisions(&mut self) {
        let collision_threshold = 150.0;

        let mut i = 0;
        let mut enems = self.enemies.borrow_mut();

        while i < self.bullets.len() {
            let bullet = &self.bullets[i];
            let bullet_position = bullet.get_position();
        
            let mut j = 0;
            
            while j < enems.len() {
                let enemy = &enems[j];
                let enemy_position = enemy.get_position();
        
                let distance = ((bullet_position.x - enemy_position.x).powi(2)
                    + (bullet_position.y - enemy_position.y).powi(2))
                    .sqrt();
        
                if distance < collision_threshold {
                    // 衝突した場合、弾と敵を削除しスコアを加算
                    enems.remove(j);
                    self.bullets.remove(i);
                    self.score += 10;
                    break; // 1つの弾が複数の敵に当たらないように
                } else {
                    j += 1;
                }
            }
            if j == enems.len() {
                i += 1;
            }
        }
    }

    fn check_player_enemy_collisions(&mut self) {
        let mut k = 0;
        let mut enems = self.enemies.borrow_mut();
    
        while k < enems.len() {
            let enemy = &enems[k];

            let collision_threshold = 120.0;
            //Logger::log(&format!("threshold: {}", collision_threshold));
    
            let enemy_position = enemy.get_position();
            let player_position = self.player.get_position();
    
            let distance = ((player_position.x - enemy_position.x).powi(2)
                + (player_position.y - enemy_position.y).powi(2))
                .sqrt();

            //Logger::log(&format!("distance: {}", distance));
    
            if distance < collision_threshold {
                // 衝突した場合、プレイヤーのライフを減らし、敵を削除
                self.player.decrease_life();
                enems.remove(k);
    
                // プレイヤーのライフが0ならゲームオーバー
                if self.player.get_life() == 0 {
                    self.state = GameState::GameOver;
                }
            } else {
                k += 1;
            }
        }
    }

    pub fn key_down(&mut self, key: String) {
        if !self.keys_pressed.contains(&key) {
            self.keys_pressed.push(key.clone());
        }

        if key == " " || key == "Space" {
            // スペースバーが押された場合、弾丸を発射
            self.fire_bullet();
        }

        if key == "ArrowRight" {
            // 右矢印キーが押された場合、プレイヤーを右に移動
            self.player.move_right(self.renderer.canvas.width() as f32);
        } 

        if key == "ArrowLeft" {
            // 左矢印キーが押された場合、プレイヤーを左に移動
            self.player.move_left(self.renderer.canvas.width() as f32);
        }
    }

    pub fn key_up(&mut self, key: String) {
        if let Some(pos) = self.keys_pressed.iter().position(|x| *x == key) {
            self.keys_pressed.remove(pos);
        }
    }

    pub fn fire_bullet(&mut self) {
        let bullet = Bullet::new(
            self.player.position.x,
            self.player.position.y
        );
        self.bullets.push(bullet);
    }

    fn render(&self) {
        self.renderer.clear();
        self.renderer.draw_background();
        self.renderer.draw_player(&self.player);
        self.renderer.draw_bullets(&self.bullets);
        self.renderer.draw_enemies(&self.enemies.borrow());
        self.renderer.draw_score(self.score);
        self.renderer.draw_life(self.player.get_life());

        // 中心座標の点を描画
        self.renderer.draw_center_points(
            &self.player,
            &self.bullets,
            &self.enemies.borrow(),
        );
    }

    pub fn game_over(&mut self) {
        web_sys::window()
            .unwrap()
            .alert_with_message("Game Over!")
            .unwrap();
    }
}