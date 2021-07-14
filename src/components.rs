use rltk::RGB;
use specs::prelude::*;
use specs_derive::Component;

use crate::gui::Options;

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: u8,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component, Clone, Copy)]
pub struct PlayerPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct ConversationAI {
    pub innocent: bool,
}

pub struct ConversationChecker {}

impl<'a> System<'a> for ConversationChecker {
    type SystemData = (
        ReadStorage<'a, ConversationAI>,
        ReadStorage<'a, Position>,
        ReadExpect<'a, PlayerPosition>,
        WriteExpect<'a, Options>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (conversables, positions, player_pos, mut options) = data;

        options.remove_option('T');

        for (pos, _conversation) in (&positions, &conversables).join() {
            if (pos.x - player_pos.x).abs() <= 1 && (pos.y - player_pos.y).abs() <= 1 {
                options.add_option('T', "Talk");
            }
        }
    }
}

#[derive(Component)]
pub struct Character {
    pub name: String,
}
