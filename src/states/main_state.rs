use legion::*;
use macroquad::prelude::*;
use macroquad::rand::*;

use super::{end_state::EndState, state::State};
use crate::{
    constants::*,
    game::{components::*, map::Map, story::Story},
};
pub struct MainState {
    font: Font,
    needs_transition: String,
    world: World,
    map: Map,
    story: Story,
}

impl MainState {
    pub fn new(font: Font) -> Self {
        let mut world = World::default();

        let _player: Entity = world.push((
            Position { x: 0.0, y: 0.0 },
            Velocity { dx: 0.0, dy: 0.0 },
            Inventory { items: [0; 27] },
            Glyph {
                char: '@',
                color: WHITE,
            },
        ));

        let story = Story::rand_gen();
        let map = Map::from_story(story.clone());
        Self {
            font,
            needs_transition: String::from("none"),
            world,
            map,
            story,
        }
    }
}

impl State for MainState {
    fn draw(&self) {
        clear_background(BLACK);
        // let params = TextParams {
        //     font: self.font,
        //     font_size: FONT_SIZE as u16,
        //     font_scale: 1.0,
        //     font_scale_aspect: 1.0,
        //     color: WHITE,
        // };
        // draw_text_ex(&format!("{:?}", self.story), 0.0, 12.0, params);

        let mut query = <(&Position, &Glyph)>::query();

        for (position, glyph) in query.iter(&self.world) {
            let params = TextParams {
                font: self.font,
                font_size: FONT_SIZE as u16,
                font_scale: 1.0,
                font_scale_aspect: 1.0,
                color: glyph.color,
            };

            draw_text_ex(
                &format!("{}", glyph.char),
                position.x * FONT_SIZE as f32
                    + (screen_width() - SCREEN_WIDTH * FONT_SIZE as f32) / 2.0,
                position.y * FONT_SIZE as f32
                    + (screen_height() - SCREEN_HEIGHT * FONT_SIZE as f32) / 2.0
                    + FONT_SIZE as f32,
                params,
            );
        }
    }

    fn handle_events(&mut self) {
        let mut query = <(&mut Velocity, &Inventory)>::query();

        for (velocity, _inventory) in query.iter_mut(&mut self.world) {
            if is_key_down(KeyCode::Right) {
                velocity.dx = 1.0;
                velocity.dy = 0.0;
            } else if is_key_down(KeyCode::Left) {
                velocity.dx = -1.0;
                velocity.dy = 0.0;
            } else if is_key_down(KeyCode::Up) {
                velocity.dx = 0.0;
                velocity.dy = -1.0;
            } else if is_key_down(KeyCode::Down) {
                velocity.dx = 0.0;
                velocity.dy = 1.0;
            } else {
                velocity.dx = 0.0;
                velocity.dy = 0.0;
            }
        }
    }

    fn tick(&mut self) {
        let mut query = <(&mut Position, &Velocity)>::query();

        for (position, velocity) in query.iter_mut(&mut self.world) {
            position.x += velocity.dx;
            position.y += velocity.dy;
        }
    }

    fn transition(&self) -> Option<Box<dyn State>> {
        match self.needs_transition.as_str() {
            "exit" => {
                let end_state = EndState::new(self.font);
                Some(Box::new(end_state))
            }
            _ => None,
        }
    }
}
