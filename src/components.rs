use rltk::RGB;
use specs::prelude::*;
use specs_derive::Component;

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

#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component, Clone, Copy)]
pub struct PlayerPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct ConversationAI {
    pub innocent: bool,
}
