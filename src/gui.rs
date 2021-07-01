use super::RunState;

use super::State;
use rltk::{Console, Rltk, RGB};

#[derive(PartialEq, Clone, Copy)]
pub enum MainMenuSelection {
    Play,
    Quit,
}

#[derive(PartialEq, Clone, Copy)]
pub enum MainMenuResult {
    NoSelection { selection: MainMenuSelection },
    Selection { selection: MainMenuSelection },
}

pub fn main_menu(gs: &mut State, ctx: &mut Rltk) -> MainMenuResult {
    let runstate = gs.ecs.fetch::<RunState>();

    ctx.print_color(
        0,
        0,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "Noir",
    );

    if let RunState::MainMenu { selection } = *runstate {
        if selection == MainMenuSelection::Play {
            ctx.print_color(
                0,
                3,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                "> Play",
            );
        } else {
            ctx.print_color(
                0,
                3,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                "  Play",
            );
        }

        if selection == MainMenuSelection::Quit {
            ctx.print_color(
                0,
                5,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                "> Quit",
            );
        } else {
            ctx.print_color(
                0,
                5,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                "  Quit",
            );
        }

        match ctx.key {
            None => return MainMenuResult::NoSelection { selection },
            Some(key) => match key {
                rltk::VirtualKeyCode::Down => {
                    let newselection = match selection {
                        MainMenuSelection::Play => MainMenuSelection::Quit,
                        MainMenuSelection::Quit => MainMenuSelection::Play,
                    };
                    return MainMenuResult::NoSelection {
                        selection: newselection,
                    };
                }
                rltk::VirtualKeyCode::Up => {
                    let newselection = match selection {
                        MainMenuSelection::Play => MainMenuSelection::Quit,
                        MainMenuSelection::Quit => MainMenuSelection::Play,
                    };
                    return MainMenuResult::NoSelection {
                        selection: newselection,
                    };
                }
                rltk::VirtualKeyCode::Return => {
                    return MainMenuResult::Selection { selection };
                }
                _ => return MainMenuResult::NoSelection { selection },
            },
        }
    }

    MainMenuResult::NoSelection {
        selection: MainMenuSelection::Play,
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum GameOverResult {
    None,
    ReturnToMain,
}

pub fn game_over(ctx: &mut Rltk) -> GameOverResult {
    ctx.print_color(
        0,
        0,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "Thank you for playing.",
    );

    ctx.print_color(
        0,
        3,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "Press any key to return to the main menu.",
    );

    match ctx.key {
        None => GameOverResult::None,
        Some(key) => GameOverResult::ReturnToMain,
    }
}
