pub mod gamepiece;
pub mod coordinate;
pub mod gameengine;
pub mod gamecanvas;

pub use self::gameengine::{GameEngine};
pub use self::gamecanvas::{GameCanvas};
use web_sys::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    // let b = GameEngine::new();

    let gamecanvas = GameCanvas::new();
    gamecanvas.initialize_canvas();
}


pub fn main() {

}
