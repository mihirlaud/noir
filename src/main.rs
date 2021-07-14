mod components;
mod constants;
mod gui;
mod map;
mod story;

use std::collections::HashMap;

use components::*;
use constants::*;
use gui::{
    draw_log, draw_sidebar, log_message, GameOverResult, MainMenuSelection, PauseMenuSelection,
};
use map::Tile;
use rltk::{Console, GameState, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;

use crate::{
    gui::{Log, Message, Options},
    map::Map,
    story::Story,
};

#[derive(PartialEq, Clone, Copy)]
enum RunState {
    MainMenu { selection: MainMenuSelection },
    AwaitingInput,
    Paused { selection: PauseMenuSelection },
    GameOver { result: GameOverResult },
}

pub struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut conversation_checker = ConversationChecker {};
        conversation_checker.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        self.run_systems();

        let mut newrunstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            newrunstate = *runstate;
        }

        match newrunstate {
            RunState::MainMenu { .. } => {}
            RunState::GameOver { .. } => {}
            _ => {
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
                let renderables = self.ecs.read_storage::<Renderable>();

                for (pos, render) in (&positions, &renderables).join() {
                    ctx.set(
                        MAP_WIDTH / 2 + 1 + pos.x - player_pos.x,
                        MAP_HEIGHT / 2 + 1 + pos.y - player_pos.y,
                        render.fg,
                        render.bg,
                        render.glyph,
                    );
                }

                draw_log(self, ctx);
                draw_sidebar(self, ctx);
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
            RunState::Paused { .. } => {
                let result = gui::pause_menu(self, ctx);

                match result {
                    gui::PauseMenuResult::NoSelection { selection } => {
                        newrunstate = RunState::Paused { selection }
                    }
                    gui::PauseMenuResult::Selection { selection } => match selection {
                        PauseMenuSelection::Return => newrunstate = RunState::AwaitingInput,
                        PauseMenuSelection::Quit => {
                            newrunstate = RunState::GameOver {
                                result: GameOverResult::None,
                            }
                        }
                    },
                };
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
    gs.ecs.register::<ConversationAI>();

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

    let hello_noir = Message::new("06:00", "Game", "Hello Noir!", RGB::named(rltk::RED));
    let player_msg = Message::new(
        "10:00",
        "You",
        "How are you today?",
        RGB::named(rltk::WHITE),
    );

    let log = Log {
        log: vec![hello_noir, player_msg],
    };
    gs.ecs.insert(log);

    let mut options = HashMap::new();
    options.insert('P', "Pause".to_string());
    options.insert('I', "Inventory".to_string());

    let options = Options { options };
    gs.ecs.insert(options);

    rltk::main_loop(context, gs)
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut pos = *ecs.fetch::<PlayerPosition>();

    let map = ecs.fetch::<Map>();

    let positions = ecs.read_storage::<Position>();

    let mut blocked = false;

    for position in (&positions).join() {
        if position.x == pos.x + delta_x && position.y == pos.y + delta_y {
            blocked = true;
            break;
        }
    }

    if !blocked && map.get_tile(pos.x + delta_x, pos.y + delta_y) == Tile::Floor {
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
            VirtualKeyCode::P => {
                return RunState::Paused {
                    selection: PauseMenuSelection::Return,
                }
            }
            VirtualKeyCode::I => {}
            VirtualKeyCode::Tab => {
                let story = gs.ecs.fetch::<Story>();
                rltk::console::log(&format!("{:?}", *story));
            }
            _ => {}
        },
    }
    RunState::AwaitingInput
}
