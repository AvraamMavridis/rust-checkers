pub mod gamepiece;
pub mod coordinate;
pub mod gameengine;

pub use self::gameengine::{GameEngine};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}


pub fn main() {
    // test();
    let b = GameEngine::new();
}
