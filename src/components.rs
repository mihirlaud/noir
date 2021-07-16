use rltk::RGB;
use specs::prelude::*;
use specs_derive::Component;

use crate::{
    gui::Options,
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
pub struct ConversationAI {
    pub innocent: bool,
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
        ReadStorage<'a, Position>,
        ReadExpect<'a, PlayerPosition>,
        WriteExpect<'a, Options>,
        WriteExpect<'a, TalkEntity>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, suspects, conversables, positions, player_pos, mut options, mut talk) = data;

        options.remove_option('T');

        for (ent, _suspect, pos, _conversation) in
            (&entities, &suspects, &positions, &conversables).join()
        {
            if (pos.x - player_pos.x).abs() <= 1 && (pos.y - player_pos.y).abs() <= 1 {
                options.add_option('T', "Talk");
                (*talk).entity = ent;
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
