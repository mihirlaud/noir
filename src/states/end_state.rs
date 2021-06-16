use super::state::State;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Clone)]
pub struct EndState {}

impl EndState {
    pub fn new() -> Self {
        Self {}
    }
}

impl State for EndState {
    fn box_clone(&self) -> Box<State> {
        Box::new((*self).clone())
    }

    fn draw(&self) -> Result<(), JsValue> {
        crate::clear_canvas();

        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        context.set_font("20px square-font");
        context.set_fill_style(&JsValue::from("white"));

        context.fill_text("THANK YOU FOR PLAYING", 0.0, 20.0)?;

        Ok(())
    }

    fn handle_events(&mut self, event: web_sys::KeyboardEvent) {}

    fn transition(&self) -> Option<String> {
        None
    }
}
