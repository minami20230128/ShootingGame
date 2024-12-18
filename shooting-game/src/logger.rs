use wasm_bindgen::prelude::*;
use web_sys;

#[wasm_bindgen]
extern{
    fn alert(s: &str);
}

pub struct Logger;

impl Logger{
    pub fn log(message: &str){
        web_sys::console::log_1(&message.into());
    }
}