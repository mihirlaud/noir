mod components;
mod constants;
mod gui;
mod map;
mod story;

use components::*;
use constants::*;
use gui::{GameOverResult, MainMenuSelection};
use map::Tile;
use rltk::{Console, GameState, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;
use std::cmp::{max, min};

use crate::{map::Map, story::Story};

#[derive(PartialEq, Clone, Copy)]
enum RunState {
    MainMenu { selection: MainMenuSelection },
    AwaitingInput,
    GameOver { result: GameOverResult },
}

pub struct State {
    ecs: World,
}

impl State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        let mut newrunstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            newrunstate = *runstate;
        }

        match newrunstate {
            RunState::MainMenu { .. } => {}
            RunState::GameOver { .. } => {}
            _ => {
                let renderables = self.ecs.read_storage::<Renderable>();

                let player_pos = self.ecs.fetch::<PlayerPosition>();
                let player_render = self.ecs.fetch::<Renderable>();

                let map = self.ecs.fetch::<Map>();
                map.draw(ctx, player_pos.x, player_pos.y);

                ctx.set(
                    MAP_WIDTH / 2 + 1,
                    MAP_HEIGHT / 2 + 1,
                    player_render.fg,
                    player_render.bg,
                    player_render.glyph,
                );

                let positions = self.ecs.read_storage::<Position>();

                for (pos, render) in (&positions, &renderables).join() {
                    ctx.set(
                        MAP_WIDTH / 2 + 1 + pos.x - player_pos.x,
                        MAP_HEIGHT / 2 + 1 + pos.y - player_pos.y,
                        render.fg,
                        render.bg,
                        render.glyph,
                    );
                }
            }
        };

        match newrunstate {
            RunState::MainMenu { .. } => {
                let result = gui::main_menu(self, ctx);
                match result {
                    gui::MainMenuResult::NoSelection { selection } => {
                        newrunstate = RunState::MainMenu { selection }
                    }
                    gui::MainMenuResult::Selection { selection } => match selection {
                        MainMenuSelection::Play => newrunstate = RunState::AwaitingInput,
                        MainMenuSelection::Quit => {
                            newrunstate = RunState::GameOver {
                                result: GameOverResult::None,
                            }
                        }
                    },
                };
            }
            RunState::AwaitingInput => {
                newrunstate = player_input(self, ctx);
            }
            RunState::GameOver { .. } => {
                let result = gui::game_over(ctx);
                match result {
                    GameOverResult::None => {
                        newrunstate = RunState::GameOver {
                            result: GameOverResult::None,
                        }
                    }
                    GameOverResult::ReturnToMain => {
                        newrunstate = RunState::MainMenu {
                            selection: MainMenuSelection::Play,
                        };
                    }
                }
            }
        };

        {
            let mut runwriter = self.ecs.write_resource::<RunState>();
            *runwriter = newrunstate;
        }
    }
}

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    use rltk::RltkBuilder;
    let context = RltkBuilder::simple(SCREEN_WIDTH, SCREEN_HEIGHT).build();

    let mut gs = State { ecs: World::new() };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<ConversationAI>();

    gs.ecs
        .create_entity()
        .with(Player {})
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::WHITE),
            bg: RGB::named(rltk::BLACK),
        })
        .build();

    gs.ecs.insert(PlayerPosition { x: 1, y: 1 });
    gs.ecs.insert(Renderable {
        glyph: rltk::to_cp437('@'),
        fg: RGB::named(rltk::WHITE),
        bg: RGB::named(rltk::BLACK),
    });

    gs.ecs.insert(RunState::MainMenu {
        selection: MainMenuSelection::Play,
    });

    let story = Story::gen_rand();
    let map = Map::from_story(&story, &mut gs);

    gs.ecs.insert(story);
    gs.ecs.insert(map);

    rltk::main_loop(context, gs)
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut pos = *ecs.fetch::<PlayerPosition>();

    let map = ecs.fetch::<Map>();

    if map.get_tile(pos.x + delta_x, pos.y + delta_y) == Tile::Floor {
        pos.x = pos.x + delta_x;
        pos.y = pos.y + delta_y;
    }

    let mut new_pos = ecs.write_resource::<PlayerPosition>();
    *new_pos = pos;
}

fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    // Player movement
    match ctx.key {
        None => {} // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            VirtualKeyCode::A => {
                let story = gs.ecs.fetch::<Story>();
                rltk::console::log(&format!("{:?}", *story));
            }
            _ => {}
        },
    }
    RunState::AwaitingInput
}
