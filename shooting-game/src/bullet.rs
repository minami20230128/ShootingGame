use wasm_bindgen::prelude::*;
use js_sys::Array;

#[wasm_bindgen]
pub struct Bullet {
    x: f32,
    y: f32,
}

#[wasm_bindgen]
impl Bullet {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Bullet {
        Bullet { x, y }
    }

    pub fn move_up(&mut self) {
        self.y -= 5.0; // 弾が上に向かって進む
    }

       // 配列として位置情報を返す
    pub fn get_position(&self) -> Array {
        let arr = Array::new();
        arr.push(&JsValue::from(self.x));
        arr.push(&JsValue::from(self.y));
        arr
    }
}

