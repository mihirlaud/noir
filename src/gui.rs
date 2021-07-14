use std::collections::BTreeMap;

use super::RunState;

use super::State;
use crate::constants::*;
use rltk::VirtualKeyCode;
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
        Some(_key) => GameOverResult::ReturnToMain,
    }
}

pub fn draw_box(ctx: &mut Rltk, rect: rltk::Rect, outline_color: RGB) {
    let x1 = rect.x1;
    let x2 = rect.x2;
    let y1 = rect.y1;
    let y2 = rect.y2;

    ctx.set(x1, y1, outline_color, RGB::named(rltk::BLACK), 218);

    ctx.set(x2 - 1, y2 - 1, outline_color, RGB::named(rltk::BLACK), 217);

    ctx.set(x2 - 1, y1, outline_color, RGB::named(rltk::BLACK), 191);

    ctx.set(x1, y2 - 1, outline_color, RGB::named(rltk::BLACK), 192);

    for x in x1 + 1..x2 - 1 {
        ctx.set(x, y1, outline_color, RGB::named(rltk::BLACK), 196);
        ctx.set(x, y2 - 1, outline_color, RGB::named(rltk::BLACK), 196);
    }

    for y in y1 + 1..y2 - 1 {
        ctx.set(x1, y, outline_color, RGB::named(rltk::BLACK), 179);
        ctx.set(x2 - 1, y, outline_color, RGB::named(rltk::BLACK), 179);
    }

    let inside = rltk::Rect::with_exact(x1 + 1, y1 + 1, x2 - 2, y2 - 2);
    ctx.fill_region(
        inside,
        rltk::to_cp437(' '),
        RGB::named(rltk::BLACK),
        RGB::named(rltk::BLACK),
    );
}

#[derive(PartialEq, Clone)]
pub struct Options {
    pub options: BTreeMap<char, String>,
}

impl Options {
    pub fn add_option(&mut self, key: char, value: &str) {
        self.options.insert(key, value.to_string());
    }

    pub fn remove_option(&mut self, key: char) {
        self.options.remove(&key);
    }
}

pub fn draw_sidebar(gs: &State, ctx: &mut Rltk) {
    let sidebar_box = rltk::Rect::with_size(MAP_WIDTH, 0, SIDEBAR_WIDTH, SIDEBAR_HEIGHT);
    draw_box(ctx, sidebar_box, RGB::named(rltk::WHITE));

    ctx.print_color(
        MAP_WIDTH + 1,
        1,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "~~ OPTIONS ~~",
    );

    let options = gs.ecs.fetch::<Options>().options.clone();
    let mut y = 3;
    for key in options.keys() {
        ctx.print_color(
            MAP_WIDTH + 1,
            y,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            &format!("{} - {}", key, options.get(key).unwrap()),
        );
        y += 2;
        if y > SIDEBAR_HEIGHT - 2 {
            break;
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Log {
    pub log: Vec<Message>,
}

#[derive(PartialEq, Clone)]
pub struct Message {
    timestamp: String,
    speaker: String,
    content: String,
    color: RGB,
}

impl Message {
    pub fn new(timestamp: &str, speaker: &str, content: &str, color: RGB) -> Self {
        Message {
            timestamp: timestamp.to_string(),
            speaker: speaker.to_string(),
            content: content.to_string(),
            color,
        }
    }

    pub fn color(&self) -> RGB {
        self.color
    }

    pub fn to_string(&self) -> String {
        format!("{} | {}: {}", self.timestamp, self.speaker, self.content)
    }
}

pub fn draw_log(gs: &State, ctx: &mut Rltk) {
    let log_box = rltk::Rect::with_size(0, MAP_HEIGHT, LOG_WIDTH, LOG_HEIGHT);
    draw_box(ctx, log_box, RGB::named(rltk::WHITE));

    ctx.print_color_centered(
        MAP_HEIGHT + 1,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "~~ LOG ~~",
    );

    let log = gs.ecs.fetch::<Log>().log.clone();
    let mut y = SCREEN_HEIGHT - 2;
    for msg in log.iter() {
        ctx.print_color(
            1,
            y,
            msg.color(),
            RGB::named(rltk::BLACK),
            msg.to_string().as_str(),
        );
        y -= 1;
        if y < MAP_HEIGHT + 2 {
            break;
        }
    }
}

pub fn log_message(gs: &mut State, msg: Message) {
    let mut log = gs.ecs.fetch::<Log>().log.clone();
    log.insert(0, msg);
    let log = Log { log };
    gs.ecs.insert(log);
}

#[derive(PartialEq, Clone, Copy)]
pub enum PauseMenuSelection {
    Return,
    Quit,
}

#[derive(PartialEq, Clone, Copy)]
pub enum PauseMenuResult {
    NoSelection { selection: PauseMenuSelection },
    Selection { selection: PauseMenuSelection },
}

pub fn pause_menu(gs: &mut State, ctx: &mut Rltk) -> PauseMenuResult {
    let x1 = SCREEN_WIDTH / 2 - 10;
    let y1 = SCREEN_HEIGHT / 2 - 5;

    let rect = rltk::Rect::with_size(x1, y1, 21, 11);
    draw_box(ctx, rect, RGB::named(rltk::WHITE));

    let runstate = gs.ecs.fetch::<RunState>();

    ctx.print_color_centered(
        y1 + 1,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "~~ PAUSE ~~",
    );

    if let RunState::Paused { selection } = *runstate {
        if selection == PauseMenuSelection::Return {
            ctx.print_color(
                x1 + 1,
                y1 + 3,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                "> RETURN",
            );
        } else {
            ctx.print_color(
                x1 + 1,
                y1 + 3,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                "  RETURN",
            );
        }

        if selection == PauseMenuSelection::Quit {
            ctx.print_color(
                x1 + 1,
                y1 + 5,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                "> QUIT",
            );
        } else {
            ctx.print_color(
                x1 + 1,
                y1 + 5,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                "  QUIT",
            );
        }

        match ctx.key {
            None => return PauseMenuResult::NoSelection { selection },
            Some(key) => match key {
                rltk::VirtualKeyCode::Down => {
                    let newselection = match selection {
                        PauseMenuSelection::Return => PauseMenuSelection::Quit,
                        PauseMenuSelection::Quit => PauseMenuSelection::Return,
                    };
                    return PauseMenuResult::NoSelection {
                        selection: newselection,
                    };
                }
                rltk::VirtualKeyCode::Up => {
                    let newselection = match selection {
                        PauseMenuSelection::Return => PauseMenuSelection::Quit,
                        PauseMenuSelection::Quit => PauseMenuSelection::Return,
                    };
                    return PauseMenuResult::NoSelection {
                        selection: newselection,
                    };
                }
                rltk::VirtualKeyCode::Return => {
                    return PauseMenuResult::Selection { selection };
                }
                _ => return PauseMenuResult::NoSelection { selection },
            },
        }
    }

    PauseMenuResult::NoSelection {
        selection: PauseMenuSelection::Return,
    }
}

pub fn draw_talk_panel(gs: &State, ctx: &mut Rltk) {
    let rect = rltk::Rect::with_size(0, 0, TALK_PANEL_WIDTH, TALK_PANEL_HEIGHT);
    draw_box(ctx, rect, RGB::named(rltk::WHITE));
}

pub enum ViewLogResult {
    None(usize),
    Up(usize),
    Down(usize),
    Exit,
}

pub fn view_log(gs: &State, ctx: &mut Rltk, page: usize) -> ViewLogResult {
    let rect = rltk::Rect::with_size(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);
    draw_box(ctx, rect, RGB::named(rltk::WHITE));

    ctx.print_color_centered(
        1,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "~~ LOG ~~",
    );

    let log = gs.ecs.fetch::<Log>().log.clone();
    let mut y = SCREEN_HEIGHT - 4;
    for i in page * (SCREEN_HEIGHT - 5) as usize..(page + 1) * (SCREEN_HEIGHT - 5) as usize {
        if log.get(i).is_some() {
            let msg = log[i].clone();
            ctx.print_color(
                1,
                y,
                msg.color(),
                RGB::named(rltk::BLACK),
                msg.to_string().as_str(),
            );
            y -= 1;
        } else {
            break;
        }
    }

    ctx.print_color(
        1,
        SCREEN_HEIGHT - 2,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "Press [Esc] to leave. Use [Up] and [Down] to scroll.",
    );

    match ctx.key {
        None => ViewLogResult::None(page),
        Some(key) => match key {
            VirtualKeyCode::Up => {
                ViewLogResult::Up(if log.len() > (page + 1) * (SCREEN_HEIGHT - 5) as usize {
                    page + 1
                } else {
                    page
                })
            }
            VirtualKeyCode::Down => ViewLogResult::Down(if page > 0 { page - 1 } else { page }),
            VirtualKeyCode::Escape => ViewLogResult::Exit,
            _ => ViewLogResult::None(page),
        },
    }
}
