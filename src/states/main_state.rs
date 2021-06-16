use crate::components::*;
use crate::game::story::Story;

use super::state::State;

use legion::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub struct MainState {
    ctx: web_sys::CanvasRenderingContext2d,
    world: World,
    player: Entity,
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

        let story = Story::rand_gen();

        let mut world = World::default();
        let player = world.push((
            Position { x: 0, y: 0 },
            Glyph {
                char: '@',
                color: "white",
            },
        ));

        Self {
            ctx: context,
            world,
            player,
            story,
            transition: "none".to_string(),
        }
    }
}

impl Clone for MainState {
    fn clone(&self) -> Self {
        let mut world_b = World::default();

        let mut merger = world::Duplicate::default();

        merger.register_copy::<Position>();
        merger.register_copy::<Velocity>();
        merger.register_copy::<Glyph>();

        world_b.clone_from(&self.world, &any(), &mut merger);

        Self {
            ctx: self.ctx.clone(),
            world: world_b,
            player: self.player.clone(),
            story: self.story.clone(),
            transition: self.transition.clone(),
        }
    }
}

impl State for MainState {
    fn box_clone(&self) -> Box<State> {
        Box::new((*self).clone())
    }

    fn draw(&self) -> Result<(), JsValue> {
        crate::clear_canvas();

        self.ctx.set_font("20px square-font");

        let mut query = <(&Position, &Glyph)>::query();

        for (position, glyph) in query.iter(&self.world) {
            self.ctx.set_fill_style(&JsValue::from(glyph.color));

            let x = position.x as f64 * 20.0;
            let y = position.y as f64 * 20.0 + 20.0;

            self.ctx.fill_text(&format!("{}", glyph.char), x, y)?;
        }

        unsafe {
            web_sys::console::log_1(&JsValue::from(format!("{:?}", self.story)));
        }

        Ok(())
    }

    fn handle_events(&mut self, event: web_sys::KeyboardEvent) {
        let player_pos = self
            .world
            .entry_mut(self.player)
            .unwrap()
            .into_component_mut::<Position>()
            .unwrap();

        match event.key_code() {
            37 => {
                player_pos.x -= 1;
            }
            38 => {
                player_pos.y -= 1;
            }
            39 => {
                player_pos.x += 1;
            }
            40 => {
                player_pos.y += 1;
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
