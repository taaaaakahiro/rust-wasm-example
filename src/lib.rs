use wasm_bindgen::prelude::*;

#[wasm_bindgen]
// js内の外部関数をRustから呼ぶ
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
// jsが呼ぶRustの関数
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
