use std::collections::BTreeMap;

use super::RunState;

use super::State;
use crate::components::ConversationAI;
use crate::constants::*;
use crate::story::Clue;
use crate::story::Suspect;
use rltk::VirtualKeyCode;
use rltk::{Console, Rltk, RGB};
use specs::Entity;
use specs::WorldExt;

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

    let title = vec![
        "b.             8      ,o888888o.       8 8888   8 888888888o.   ".to_string(),
        "888o.          8   . 8888     `88.     8 8888   8 8888    `88.  ".to_string(),
        "Y88888o.       8  ,8 8888       `8b    8 8888   8 8888     `88  ".to_string(),
        ".`Y888888o.    8  88 8888        `8b   8 8888   8 8888     ,88  ".to_string(),
        "8o. `Y888888o. 8  88 8888         88   8 8888   8 8888.   ,88'  ".to_string(),
        "8`Y8o. `Y88888o8  88 8888         88   8 8888   8 888888888P'   ".to_string(),
        "8   `Y8o. `Y8888  88 8888        ,8P   8 8888   8 8888`8b       ".to_string(),
        "8      `Y8o. `Y8  `8 8888       ,8P    8 8888   8 8888 `8b.     ".to_string(),
        "8         `Y8o.`   ` 8888     ,88'     8 8888   8 8888   `8b.   ".to_string(),
        "8            `Yo      `8888888P'       8 8888   8 8888     `88. ".to_string(),
    ];

    let mut y = SCREEN_HEIGHT / 2 - 5;
    for row in title {
        ctx.print_color_centered(y, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), &row);
        y += 1;
    }

    if let RunState::MainMenu { selection } = *runstate {
        if selection == MainMenuSelection::Play {
            ctx.print_color_centered(
                SCREEN_HEIGHT / 2 + 8,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                "> Play",
            );
        } else {
            ctx.print_color_centered(
                SCREEN_HEIGHT / 2 + 8,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                "  Play",
            );
        }

        if selection == MainMenuSelection::Quit {
            ctx.print_color_centered(
                SCREEN_HEIGHT / 2 + 10,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                "> Quit",
            );
        } else {
            ctx.print_color_centered(
                SCREEN_HEIGHT / 2 + 10,
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
    ctx.print_color_centered(
        SCREEN_HEIGHT / 2 - 1,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "Thank you for playing.",
    );

    ctx.print_color_centered(
        SCREEN_HEIGHT / 2 + 1,
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

#[derive(PartialEq, Clone)]
pub struct Time {
    day: i32,
    hour: i32,
    minute: i32,
}

impl Time {
    pub fn new() -> Self {
        Time {
            day: 1,
            hour: 8,
            minute: 0,
        }
    }

    pub fn increment_day(&mut self) {
        self.day += 1;
    }

    pub fn set_time(&mut self, hour: i32, minute: i32) {
        self.hour = hour;
        self.minute = minute;
    }

    pub fn advance_hour(&mut self) {
        self.hour += 1;
        if self.hour > 23 {
            self.hour = 0;
            self.increment_day();
        }
    }

    pub fn advance_minute(&mut self) {
        self.minute += 1;
        if self.minute > 59 {
            self.minute = 0;
            self.advance_hour();
        }
    }

    pub fn get_day(&self) -> i32 {
        self.day
    }

    pub fn get_time(&self) -> String {
        let hour = if self.hour > 9 {
            format!("{}", self.hour)
        } else {
            format!("0{}", self.hour)
        };

        let minute = if self.minute > 9 {
            format!("{}", self.minute)
        } else {
            format!("0{}", self.minute)
        };

        format!("{}:{}", hour, minute)
    }

    pub fn get_day_and_time(&self) -> String {
        format!("Day {}, {}", self.day, self.get_time())
    }
}

pub fn draw_sidebar(gs: &State, ctx: &mut Rltk) {
    let sidebar_box = rltk::Rect::with_size(MAP_WIDTH, 0, SIDEBAR_WIDTH, SIDEBAR_HEIGHT);
    draw_box(ctx, sidebar_box, RGB::named(rltk::WHITE));

    ctx.print_color(
        MAP_WIDTH + 1,
        7,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "~~ OPTIONS ~~",
    );

    let options = gs.ecs.fetch::<Options>().options.clone();
    let mut y = 9;
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

    let time_box = rltk::Rect::with_size(MAP_WIDTH + 4, 3, 7, 3);
    draw_box(ctx, time_box, RGB::named(rltk::WHITE));

    let time = gs.ecs.fetch::<Time>();

    ctx.print_color(
        MAP_WIDTH + 5,
        2,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        &format!("DAY {}", time.get_day()),
    );

    ctx.print_color(
        MAP_WIDTH + 5,
        4,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        &time.get_time(),
    );
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
        y -= 2;
        if y < MAP_HEIGHT + 2 {
            break;
        }
    }
}

pub fn log_message(gs: &mut State, speaker: &str, content: &str, color: RGB) {
    let timestamp;
    {
        let time = gs.ecs.fetch::<Time>();
        timestamp = time.get_day_and_time();
    }

    let msg = Message::new(timestamp.as_str(), speaker, content, color);

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

pub fn draw_talk_panel(gs: &mut State, ctx: &mut Rltk) -> RunState {
    let rect = rltk::Rect::with_size(0, 0, TALK_PANEL_WIDTH, TALK_PANEL_HEIGHT);
    draw_box(ctx, rect, RGB::named(rltk::WHITE));

    let entity = gs.ecs.read_resource::<Entity>();

    let mut speaker_store = gs.ecs.write_storage::<Suspect>();
    let mut speaker = speaker_store.get_mut(*entity).unwrap();

    let headshot = rltk::Rect::with_size(2, 2, 11, 11);
    draw_box(ctx, headshot, RGB::named(rltk::WHITE));

    ctx.print_color(
        14,
        3,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "Name:",
    );
    ctx.print_color(20, 3, speaker.color, RGB::named(rltk::BLACK), &speaker.name);

    ctx.print_color(
        14,
        6,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "Age:",
    );
    ctx.print_color(
        19,
        6,
        speaker.color,
        RGB::named(rltk::BLACK),
        &format!("{}", speaker.age),
    );

    ctx.set(0, 14, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), 195);
    ctx.set(
        TALK_PANEL_WIDTH - 1,
        14,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        180,
    );

    for x in 1..TALK_PANEL_WIDTH - 1 {
        ctx.set(x, 14, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), 196);
    }

    ctx.set(
        0,
        TALK_PANEL_HEIGHT - 3,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        195,
    );
    ctx.set(
        TALK_PANEL_WIDTH - 1,
        TALK_PANEL_HEIGHT - 3,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        180,
    );

    for x in 1..TALK_PANEL_WIDTH - 1 {
        ctx.set(
            x,
            TALK_PANEL_HEIGHT - 3,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            196,
        );
    }

    let mut ai_store = gs.ecs.write_storage::<ConversationAI>();
    let mut ai = ai_store.get_mut(*entity).unwrap();
    let options = generate_conversation_options(&speaker, *ai);

    let mut y = 16;
    for option in &options {
        ctx.print_color(
            2,
            y,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            &format!("{}. {}", (y - 14) / 2, option.0),
        );
        y += 2;
    }

    ctx.print_color(
        1,
        TALK_PANEL_HEIGHT - 2,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "Use [0-9] to talk to the suspect. Press [Esc] to leave the conversation.",
    );

    let mut idx: Option<usize> = None;

    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Key1 => idx = Some(0),
            VirtualKeyCode::Key2 => idx = Some(1),
            VirtualKeyCode::Escape => {
                return RunState::AwaitingInput;
            }
            _ => {}
        },
    }

    if idx.is_some() {
        let idx = idx.unwrap();
        let mut log = gs.ecs.write_resource::<Log>();
        let msg1 = Message::new(
            gs.ecs.fetch::<Time>().get_day_and_time().as_str(),
            "You",
            options[idx].0.as_str(),
            RGB::named(rltk::WHITE),
        );
        let msg2 = Message::new(
            gs.ecs.fetch::<Time>().get_day_and_time().as_str(),
            &speaker.name,
            options[idx].1.as_str(),
            speaker.color,
        );
        log.log.insert(0, msg1);
        log.log.insert(0, msg2);

        let mut time = gs.ecs.write_resource::<Time>();
        time.advance_minute();
    }

    RunState::Talking
}

fn generate_conversation_options(speaker: &Suspect, ai: ConversationAI) -> Vec<(String, String)> {
    let mut options = vec![];

    options.push((
        "Hello. What is your name?".to_string(),
        format!("My name is {}", speaker.name),
    ));

    options.push((
        "Are you innocent?".to_string(),
        format!("I am {}", if ai.innocent { "innocent!" } else { "guilty!" }),
    ));

    options
}

pub fn draw_examination_panel(gs: &mut State, ctx: &mut Rltk) -> RunState {
    let rect = rltk::Rect::with_size(0, 0, EXAM_PANEL_WIDTH, EXAM_PANEL_HEIGHT);
    draw_box(ctx, rect, RGB::named(rltk::WHITE));

    let entity = gs.ecs.read_resource::<Entity>();

    let mut clue_store = gs.ecs.write_storage::<Clue>();
    let clue = clue_store.get_mut(*entity).unwrap();

    ctx.print_color(
        2,
        2,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "Examining:",
    );

    display_clue(ctx, clue);

    ctx.print_color(
        13,
        2,
        clue.color,
        RGB::named(rltk::BLACK),
        &clue.name.to_uppercase(),
    );

    ctx.print_color(
        1,
        EXAM_PANEL_HEIGHT - 2,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "Use your mouse to examine the object. Press [Esc] to finish your examination.",
    );

    ctx.set(
        0,
        EXAM_PANEL_HEIGHT - 3,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        195,
    );
    ctx.set(
        EXAM_PANEL_WIDTH - 1,
        EXAM_PANEL_HEIGHT - 3,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        180,
    );

    for x in 1..EXAM_PANEL_WIDTH - 1 {
        ctx.set(
            x,
            EXAM_PANEL_HEIGHT - 3,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            196,
        );
    }

    let (mouse_x, mouse_y) = ctx.mouse_pos();
    for marker in clue.markers.clone() {
        if (mouse_x - marker.0).abs() <= 1 && (mouse_y - marker.1).abs() <= 1 {
            ctx.set(
                marker.0,
                marker.1,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                rltk::to_cp437('*'),
            );
            if ctx.left_click {
                let msg = Message::new(
                    gs.ecs.fetch::<Time>().get_day_and_time().as_str(),
                    "You",
                    marker.2.as_str(),
                    RGB::named(rltk::WHITE),
                );
                let mut log = gs.ecs.write_resource::<Log>();
                log.log.insert(0, msg);
            }
        }
    }

    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Escape => {
                return RunState::AwaitingInput;
            }
            _ => {}
        },
    }

    RunState::Examining
}

fn display_clue(ctx: &mut Rltk, clue: &Clue) {
    let display = clue.display.clone();
    let mut y = EXAM_PANEL_HEIGHT / 2 - display.len() as i32 / 2;
    for row in display {
        ctx.print_color(
            EXAM_PANEL_WIDTH / 2 - row.len() as i32 / 2,
            y,
            clue.color,
            RGB::named(rltk::BLACK),
            &row,
        );
        y += 1;
    }
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
    let mut y = SCREEN_HEIGHT - 5;
    for i in
        page * ((SCREEN_HEIGHT - 5) / 2) as usize..(page + 1) * ((SCREEN_HEIGHT - 5) / 2) as usize
    {
        if log.get(i).is_some() {
            let msg = log[i].clone();
            ctx.print_color(
                1,
                y,
                msg.color(),
                RGB::named(rltk::BLACK),
                msg.to_string().as_str(),
            );
            y -= 2;
        } else {
            break;
        }
    }

    ctx.print_color(
        1,
        SCREEN_HEIGHT - 2,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "Use [Up] and [Down] to scroll. Press [Esc] to leave.",
    );

    match ctx.key {
        None => ViewLogResult::None(page),
        Some(key) => match key {
            VirtualKeyCode::Up => ViewLogResult::Up(
                if log.len() > (page + 1) * ((SCREEN_HEIGHT - 5) / 2) as usize {
                    page + 1
                } else {
                    page
                },
            ),
            VirtualKeyCode::Down => ViewLogResult::Down(if page > 0 { page - 1 } else { page }),
            VirtualKeyCode::Escape => ViewLogResult::Exit,
            _ => ViewLogResult::None(page),
        },
    }
}
