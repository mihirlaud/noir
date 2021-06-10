use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Inventory {
    pub items: [i32; 27],
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Glyph {
    pub char: char,
    pub color: Color,
}
