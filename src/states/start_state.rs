use super::state::State;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub struct StartState {
    transition: String,
    selected: i32,
}

impl StartState {
    pub fn new() -> Self {
        Self {
            transition: "none".to_string(),
            selected: 1,
        }
    }
}

impl State for StartState {
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

        context.fill_text("NOIR", 0.0, 20.0)?;

        let play_text = if self.selected == 1 {
            "> PLAY"
        } else {
            "  PLAY"
        };
        let exit_text = if self.selected == 1 {
            "  EXIT"
        } else {
            "> EXIT"
        };

        context.fill_text(play_text, 0.0, 60.0)?;
        context.fill_text(exit_text, 0.0, 100.0)?;

        Ok(())
    }

    fn handle_events(&mut self, event: web_sys::KeyboardEvent) {
        match event.key_code() {
            38 => {
                self.selected -= 1;
                if self.selected < 1 {
                    self.selected = 2;
                }
            }
            40 => {
                self.selected += 1;
                if self.selected > 2 {
                    self.selected = 1;
                }
            }
            13 => {
                if self.selected == 1 {
                    self.transition = "play".to_string();
                } else if self.selected == 2 {
                    self.transition = "exit".to_string();
                }
            }
            _ => {}
        };
    }

    fn transition(&self) -> Option<String> {
        match self.transition.as_str() {
            "play" => Some("transition-main".to_string()),
            "exit" => Some("transition-exit".to_string()),
            _ => None,
        }
    }
}
