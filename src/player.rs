use crate::components::*;
use crate::gui;
use crate::gui::Message;
use crate::gui::Options;
use crate::gui::PauseMenuSelection;
use crate::map::Map;
use crate::map::Tile;
use crate::story::Story;
use crate::RunState;
use crate::State;
use rltk::{Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
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

pub fn input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    // Player movement
    match ctx.key {
        None => {} // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            VirtualKeyCode::P => {
                if gs.ecs.fetch::<Options>().options.contains_key(&'P') {
                    return RunState::Paused {
                        selection: PauseMenuSelection::Return,
                    };
                }
            }
            VirtualKeyCode::L => {
                if gs.ecs.fetch::<Options>().options.contains_key(&'L') {
                    return RunState::Log { page: 0 };
                }
            }
            VirtualKeyCode::T => {
                if gs.ecs.fetch::<Options>().options.contains_key(&'T') {
                    let mut speaker = "Game".to_string();
                    let mut content = "Hello Detective.";
                    {
                        let player_pos = gs.ecs.fetch::<PlayerPosition>();

                        let positions = gs.ecs.read_storage::<Position>();
                        let conversables = gs.ecs.read_storage::<ConversationAI>();
                        let characters = gs.ecs.read_storage::<Character>();

                        for (pos, con, character) in (&positions, &conversables, &characters).join()
                        {
                            if (pos.x - player_pos.x).abs() <= 1
                                && (pos.y - player_pos.y).abs() <= 1
                            {
                                speaker = character.name.clone();
                                content = if con.innocent {
                                    "Hello Detective. I am innocent!"
                                } else {
                                    "Hello Detective. I am guilty!"
                                };
                                break;
                            }
                        }
                    }
                    let msg =
                        Message::new("10:00", speaker.as_str(), content, RGB::named(rltk::WHITE));

                    gui::log_message(gs, msg);
                    return RunState::Talking;
                }
            }
            VirtualKeyCode::Tab => {
                let story = gs.ecs.fetch::<Story>();
                rltk::console::log(&format!("{:?}", *story));
            }
            _ => {}
        },
    }
    RunState::AwaitingInput
}
