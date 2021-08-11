use rltk::RGB;
use specs::prelude::*;
use specs_derive::Component;

use crate::{
    gui::Options,
    map::{Map, Tile},
    story::{Clue, Suspect},
};

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

#[derive(Component, Clone, Copy, Default)]
pub struct MovementAI {
    pub is_idle: bool,
    pub cooldown: i32,
    pub max_cooldown: i32,
}

pub struct MovementChecker {}

impl<'a> System<'a> for MovementChecker {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, MovementAI>,
        ReadExpect<'a, Map>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut positions, mut ais, map) = data;

        for (pos, ai) in (&mut positions, &mut ais).join() {
            if !ai.is_idle {
                if ai.cooldown > 0 {
                    ai.cooldown -= 1;
                } else {
                    ai.cooldown = ai.max_cooldown;

                    let mut rng = rltk::RandomNumberGenerator::new();

                    let dx = rng.range(-1, 2);
                    let dy = rng.range(-1, 2);

                    if map.get_tile(pos.x + dx, pos.y + dy) == Tile::Floor {
                        pos.x = pos.x + dx;
                        pos.y = pos.y + dy;
                    }
                }
            }
        }
    }
}

#[derive(Component, Clone, Copy, Default)]
pub struct ConversationAI {
    pub innocent: bool,
    pub evidence_hair: bool,
    pub evidence_shoe_size: bool,
}

#[derive(Component, Clone, Copy)]
pub struct TalkEntity {
    pub entity: Entity,
}

pub struct ConversationChecker {}

impl<'a> System<'a> for ConversationChecker {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Suspect>,
        ReadStorage<'a, ConversationAI>,
        WriteStorage<'a, MovementAI>,
        ReadStorage<'a, Position>,
        ReadExpect<'a, PlayerPosition>,
        WriteExpect<'a, Options>,
        WriteExpect<'a, TalkEntity>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            suspects,
            conversables,
            mut move_ai,
            positions,
            player_pos,
            mut options,
            mut talk,
        ) = data;

        options.remove_option('T');

        for (ent, _suspect, pos, _conversation, ai) in (
            &entities,
            &suspects,
            &positions,
            &conversables,
            &mut move_ai,
        )
            .join()
        {
            if (pos.x - player_pos.x).abs() <= 1 && (pos.y - player_pos.y).abs() <= 1 {
                options.add_option('T', "Talk");
                ai.is_idle = true;
                (*talk).entity = ent;
            } else {
                ai.is_idle = false;
            }
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct ExamEntity {
    pub entity: Entity,
}

pub struct ExaminationChecker {}

impl<'a> System<'a> for ExaminationChecker {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Clue>,
        ReadStorage<'a, Position>,
        ReadExpect<'a, PlayerPosition>,
        WriteExpect<'a, Options>,
        WriteExpect<'a, ExamEntity>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, clues, positions, player_pos, mut options, mut exam) = data;

        options.remove_option('X');

        for (ent, _clue, pos) in (&entities, &clues, &positions).join() {
            if (pos.x - player_pos.x).abs() <= 1 && (pos.y - player_pos.y).abs() <= 1 {
                options.add_option('X', "Examine");
                (*exam).entity = ent;
            }
        }
    }
}
