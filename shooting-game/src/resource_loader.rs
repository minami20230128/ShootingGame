use std::{collections::HashMap, rc::Rc};

use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, HtmlImageElement};

#[derive (Clone)]
pub struct ResourceLoader{
    images: HashMap<String, Rc<HtmlImageElement>>
}

impl ResourceLoader {
    pub fn new() -> ResourceLoader {
        return ResourceLoader{ images: HashMap::new() };
    }

    pub fn load_images(&mut self){
        let window = window().expect("no global `window` exists");
        let document = window.document().unwrap();

        let image_ids = vec![
            "player",
            "bullet",
            "enemy", 
            "heart", 
            "background",
        ];

        for id in image_ids {
            if let Some(img_element) = document.get_element_by_id(id) {
                // img要素をHtmlImageElementとしてキャスト
                let img = img_element
                    .dyn_into::<HtmlImageElement>()
                    .map_err(|_| JsValue::from("Failed to cast to HtmlImageElement"));
                self.images.insert(id.to_string(), Rc::new(img.unwrap()));
            }
        }
    }

    pub fn get_image(&self, name :&str) -> Rc<HtmlImageElement>{
        if let Some(image) = self.images.get(name){
            return image.clone();
        } else {
            println!("Image element not found");
            return Rc::new(HtmlImageElement::new().unwrap());
        }
    }
}