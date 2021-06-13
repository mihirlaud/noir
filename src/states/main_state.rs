use crate::game::story::Story;

use super::state::State;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub struct MainState {
    ctx: web_sys::CanvasRenderingContext2d,
    player: Player,
    story: Story,
    transition: String,
}

impl MainState {
    pub fn new() -> Self {
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

        let player = Player::new();
        let story = Story::rand_gen();

        Self {
            ctx: context,
            player,
            story,
            transition: "none".to_string(),
        }
    }
}

impl State for MainState {
    fn draw(&self) -> Result<(), JsValue> {
        crate::clear_canvas();
        self.player.draw()?;

        self.ctx.set_font("20px square-font");
        self.ctx.set_fill_style(&JsValue::from("white"));

        unsafe {
            web_sys::console::log_1(&JsValue::from(format!("{:?}", self.story)));
        }

        Ok(())
    }

    fn handle_events(&mut self, event: web_sys::KeyboardEvent) {
        match event.key_code() {
            37 => {
                self.player.move_by(-1, 0);
            }
            38 => {
                self.player.move_by(0, -1);
            }
            39 => {
                self.player.move_by(1, 0);
            }
            40 => {
                self.player.move_by(0, 1);
            }
            _ => {}
        };
    }

    fn transition(&self) -> Option<String> {
        match self.transition.as_str() {
            "main" => Some("transition-main".to_string()),
            _ => None,
        }
    }
}

pub struct Player {
    x: i32,
    y: i32,
}

impl Player {
    pub fn new() -> Self {
        let x = 0;
        let y = 0;
        Self { x, y }
    }

    pub fn draw(&self) -> Result<(), JsValue> {
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
        context.fill_text("@", self.x as f64 * 20.0, self.y as f64 * 20.0 + 20.0)?;

        Ok(())
    }

    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}
