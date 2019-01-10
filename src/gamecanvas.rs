extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use web_sys::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::Clamped;
use web_sys::{HtmlCanvasElement};

#[derive(Debug)]
pub struct GameCanvas {
    pub canvas: HtmlCanvasElement
}

impl GameCanvas {
  pub fn new() -> GameCanvas {
    let document = window().unwrap().document().unwrap();
    let _invisible_canvas: web_sys::HtmlCanvasElement = document.create_element("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    GameCanvas {
      canvas: _invisible_canvas
    }
  }

  pub fn initialize_canvas(&self) {
    let document = window().unwrap().document().unwrap();
    let body = document.query_selector(&String::from("body")).unwrap().unwrap();
    let body_node: &web_sys::Node = wasm_bindgen::JsCast::dyn_ref(&body).unwrap();
    let _apended_canvas = body_node.append_child(&self.canvas);
  }
}
