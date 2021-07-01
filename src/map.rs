use rltk::Console;
use rltk::Rltk;
use rltk::RGB;
use specs::{Builder, WorldExt};

use crate::components::*;
use crate::constants::*;
use crate::{story::Story, State};

#[derive(Clone)]
pub struct Map {
    tiles: Vec<Tile>,
}

impl Map {
    pub fn from_story(story: &Story, gs: &mut State) -> Self {
        let mut x = 5;
        for suspect in story.suspects.iter() {
            gs.ecs
                .create_entity()
                .with(Position { x, y: 10 })
                .with(Renderable {
                    glyph: rltk::to_cp437(suspect.name.chars().nth(0).unwrap()),
                    fg: if suspect.is_killer {
                        rltk::RGB::named(rltk::RED)
                    } else {
                        rltk::RGB::named(rltk::YELLOW)
                    },
                    bg: rltk::RGB::named(rltk::BLACK),
                })
                .with(ConversationAI {
                    innocent: suspect.is_killer,
                })
                .build();

            x += 1;
        }

        let tiles = vec![Tile::Floor; (MAP_WIDTH * MAP_HEIGHT) as usize];

        let mut map = Map { tiles };

        for x in 0..MAP_WIDTH {
            map.set_tile(x, 0, Tile::Wall);
            map.set_tile(x, MAP_HEIGHT - 1, Tile::Wall);
        }

        for y in 0..MAP_HEIGHT {
            map.set_tile(0, y, Tile::Wall);
            map.set_tile(MAP_WIDTH - 1, y, Tile::Wall);
        }

        map
    }

    pub fn draw(&self, ctx: &mut Rltk, player_x: i32, player_y: i32) {
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let screen_x = MAP_WIDTH / 2 + 1 + x - player_x;
                let screen_y = MAP_HEIGHT / 2 + 1 + y - player_y;

                if (0..MAP_WIDTH).contains(&screen_x) && (0..MAP_HEIGHT).contains(&screen_y) {
                    ctx.set(
                        screen_x,
                        screen_y,
                        RGB::named(rltk::GRAY),
                        RGB::named(rltk::BLACK),
                        match self.get_tile(x, y) {
                            Tile::Floor => rltk::to_cp437('.'),
                            Tile::Wall => rltk::to_cp437('#'),
                        },
                    );
                }
            }
        }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Tile {
        self.tiles[(y * MAP_WIDTH + x) as usize].clone()
    }

    pub fn set_tile(&mut self, x: i32, y: i32, tile: Tile) {
        self.tiles[(y * MAP_WIDTH + x) as usize] = tile;
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Floor,
}
