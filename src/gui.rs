use std::collections::BTreeMap;
use std::collections::HashSet;

use super::RunState;

use super::State;
use crate::components::ConversationAI;
use crate::components::ExamEntity;
use crate::components::TalkEntity;
use crate::constants::*;
use crate::story::Clue;
use crate::story::Connection;
use crate::story::Note;
use crate::story::PlayerNotes;
use crate::story::Suspect;
use rltk::VirtualKeyCode;
use rltk::{Console, Rltk, RGB};
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

impl Log {
    pub fn log_message(&mut self, time: &Time, speaker: &str, content: &str, color: RGB) {
        let timestamp = time.get_day_and_time();

        let msg = Message::new(timestamp.as_str(), speaker, content, color);

        self.log.insert(0, msg);
    }
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

    let entity = gs.ecs.read_resource::<TalkEntity>();
    let entity = (*entity).entity;

    let mut speaker_store = gs.ecs.write_storage::<Suspect>();
    let mut speaker = speaker_store.get_mut(entity).unwrap();

    let headshot = rltk::Rect::with_size(2, 2, 11, 11);
    draw_box(ctx, headshot, RGB::named(rltk::WHITE));

    ctx.print_color(
        14,
        3,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "Name:",
    );
    ctx.print_color(
        20,
        3,
        RGB::named(speaker.color),
        RGB::named(rltk::BLACK),
        &speaker.name,
    );

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
        RGB::named(speaker.color),
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
    let mut ai = ai_store.get_mut(entity).unwrap();
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
            VirtualKeyCode::Key3 => idx = Some(2),
            VirtualKeyCode::Key4 => idx = Some(3),
            VirtualKeyCode::Key5 => idx = Some(4),
            VirtualKeyCode::Key6 => idx = Some(5),
            VirtualKeyCode::Key7 => idx = Some(6),
            VirtualKeyCode::Key8 => idx = Some(7),
            VirtualKeyCode::Key9 => idx = Some(8),
            VirtualKeyCode::Key0 => idx = Some(9),
            VirtualKeyCode::Escape => {
                return RunState::AwaitingInput;
            }
            _ => {}
        },
    }

    if idx.is_some() {
        let idx = idx.unwrap();
        let mut time = gs.ecs.write_resource::<Time>();
        let mut log = gs.ecs.write_resource::<Log>();
        log.log_message(
            &time,
            "You",
            options[idx].0.as_str(),
            RGB::named(rltk::WHITE),
        );
        log.log_message(
            &time,
            &speaker.name,
            options[idx].1.as_str(),
            RGB::named(speaker.color),
        );

        time.advance_minute();

        if options[idx].2.is_some() {
            let mut notes = gs.ecs.write_resource::<PlayerNotes>();

            notes.add_note(options[idx].2.clone().unwrap());
        }
    }

    RunState::Talking
}

fn generate_conversation_options(
    speaker: &Suspect,
    ai: ConversationAI,
) -> Vec<(String, String, Option<Note>)> {
    let mut options = vec![];

    for option in speaker.convo_options.clone() {
        options.push(option.clone());
    }

    options
}

pub fn draw_examination_panel(gs: &mut State, ctx: &mut Rltk) -> RunState {
    let rect = rltk::Rect::with_size(0, 0, EXAM_PANEL_WIDTH, EXAM_PANEL_HEIGHT);
    draw_box(ctx, rect, RGB::named(rltk::WHITE));

    let entity = gs.ecs.read_resource::<ExamEntity>();
    let entity = (*entity).entity;

    let mut clue_store = gs.ecs.write_storage::<Clue>();
    let clue = clue_store.get_mut(entity).unwrap();

    ctx.print_color(
        1,
        1,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "Examining:",
    );

    ctx.set(0, 2, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), 195);
    ctx.set(
        EXAM_PANEL_WIDTH - 1,
        2,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        180,
    );

    for x in 1..EXAM_PANEL_WIDTH - 1 {
        ctx.set(x, 2, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), 196);
    }

    display_clue(ctx, clue);

    ctx.print_color(
        12,
        1,
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
    for i in 0..clue.markers.len() {
        let marker = clue.markers[i].clone();
        if (mouse_x - marker.0).abs() <= 1 && (mouse_y - marker.1).abs() <= 1 {
            ctx.set(
                marker.0,
                marker.1,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                rltk::to_cp437('*'),
            );
            if ctx.left_click {
                let mut time = gs.ecs.write_resource::<Time>();
                let mut log = gs.ecs.write_resource::<Log>();
                log.log_message(
                    &time,
                    "You",
                    marker.2.get_log_msg().as_str(),
                    RGB::named(rltk::WHITE),
                );

                clue.reveal_marker(i);

                let mut notes = gs.ecs.write_resource::<PlayerNotes>();
                notes.add_note(marker.2.clone());

                time.advance_minute();
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

    for marker in clue.markers.clone() {
        if marker.3 {
            ctx.set(
                marker.0,
                marker.1,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                rltk::to_cp437('*'),
            );
        }
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

    ctx.set(
        0,
        SCREEN_HEIGHT - 3,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        195,
    );
    ctx.set(
        SCREEN_WIDTH - 1,
        SCREEN_HEIGHT - 3,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        180,
    );

    for x in 1..SCREEN_WIDTH - 1 {
        ctx.set(
            x,
            SCREEN_HEIGHT - 3,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            196,
        );
    }

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

pub struct NoteBoxes {
    pub boxes: HashSet<u32>,
}

impl NoteBoxes {
    pub fn new() -> Self {
        NoteBoxes {
            boxes: HashSet::new(),
        }
    }

    pub fn add_box(&mut self, id: u32) {
        self.boxes.insert(id);
    }

    pub fn remove_box(&mut self, id: u32) {
        self.boxes.remove(&id);
    }

    pub fn contains_id(&self, id: u32) -> bool {
        self.boxes.contains(&id)
    }
}

pub fn draw_notes(gs: &mut State, ctx: &mut Rltk) -> RunState {
    let rect = rltk::Rect::with_size(0, 0, NOTES_PANEL_WIDTH, NOTES_PANEL_HEIGHT);
    draw_box(ctx, rect, RGB::named(rltk::WHITE));

    ctx.print_color(
        1,
        1,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "Notes",
    );

    ctx.set(0, 2, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), 195);
    ctx.set(
        NOTES_PANEL_WIDTH - 1,
        2,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        180,
    );

    for x in 1..NOTES_PANEL_WIDTH - 1 {
        ctx.set(x, 2, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), 196);
    }

    ctx.print_color(
        1,
        NOTES_PANEL_HEIGHT - 2,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "Use [Up] and [Down] to scroll through notes. Press [Esc] to leave notes.",
    );

    ctx.set(
        0,
        NOTES_PANEL_HEIGHT - 3,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        195,
    );
    ctx.set(
        NOTES_PANEL_WIDTH - 1,
        NOTES_PANEL_HEIGHT - 3,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        180,
    );

    for x in 1..NOTES_PANEL_WIDTH - 1 {
        ctx.set(
            x,
            NOTES_PANEL_HEIGHT - 3,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            196,
        );
    }

    let mut note_boxes = gs.ecs.write_resource::<NoteBoxes>();

    let mut click_zones: Vec<(i32, i32, i32, u32)> = vec![];

    let mut notes = gs.ecs.write_resource::<PlayerNotes>();

    let mut y = 4;
    for note in notes.notes.iter() {
        ctx.print_color(2, y, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), "o");
        let mut x = 4;
        for pair in note.note.clone() {
            ctx.print_color(
                x,
                y,
                RGB::named(pair.1),
                RGB::named(rltk::BLACK),
                &format!("{}", pair.0),
            );
            if pair.2 {
                click_zones.push((x, y, pair.0.len() as i32, note.id));
                if note_boxes.contains_id(note.id) {
                    rltk::draw_hollow_box(
                        ctx,
                        x - 1,
                        y - 1,
                        pair.0.len() as i32 + 1,
                        2,
                        RGB::named(rltk::WHITE),
                        RGB::named(rltk::BLACK),
                    );
                }
            }
            x += pair.0.len() as i32 + 1;
        }
        y += 2;
    }

    if ctx.left_click {
        let (mouse_x, mouse_y) = ctx.mouse_pos();
        for click_zone in click_zones {
            if mouse_y == click_zone.1
                && (click_zone.0..click_zone.0 + click_zone.2).contains(&mouse_x)
            {
                if note_boxes.contains_id(click_zone.3) {
                    note_boxes.remove_box(click_zone.3);
                } else {
                    note_boxes.add_box(click_zone.3);
                }
            }
        }
    }

    let mut cxns = gs.ecs.write_resource::<Vec<Connection>>();

    if note_boxes.boxes.len() >= 2 {
        let mut time = gs.ecs.write_resource::<Time>();
        let mut log = gs.ecs.write_resource::<Log>();
        let mut cxn_found = false;

        for i in 0..cxns.len() {
            let cxn = cxns[i].clone();
            if note_boxes.contains_id(cxn.ids.0) && note_boxes.contains_id(cxn.ids.1) {
                cxn_found = true;
                let new_note = cxn.note;
                log.log_message(
                    &time,
                    "You",
                    new_note.get_log_msg().as_str(),
                    RGB::named(rltk::GREEN),
                );
                notes.add_note(new_note);
                cxns.remove(i);

                let mut options = gs.ecs.write_resource::<Options>();

                options.add_option('A', "Accuse");

                break;
            }
        }

        if !cxn_found {
            log.log_message(
                &time,
                "You",
                "I can't think of a connection between these...",
                RGB::named(rltk::WHITE),
            );
        }

        time.advance_minute();

        note_boxes.boxes.clear();
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

    RunState::Notes
}

pub fn draw_accuse_panel(gs: &mut State, ctx: &mut Rltk) -> RunState {
    let rect = rltk::Rect::with_size(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);
    draw_box(ctx, rect, RGB::named(rltk::WHITE));

    ctx.print_color(
        1,
        1,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "ACCUSE",
    );

    ctx.print_color(
        1,
        SCREEN_HEIGHT - 2,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "Use your mouse to answer the questions. Press [Esc] to leave.",
    );

    ctx.print_color(
        2,
        5,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "Who is the killer?",
    );

    ctx.print_color(
        2,
        8,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "What is the murder weapon?",
    );

    ctx.print_color(
        2,
        11,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "Provide one piece of evidence: ",
    );

    ctx.print_color(
        2,
        14,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "Provide another piece of evidence: ",
    );

    ctx.set(0, 17, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), 195);
    ctx.set(
        SCREEN_WIDTH - 1,
        17,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        180,
    );

    for x in 1..SCREEN_WIDTH - 1 {
        ctx.set(x, 17, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), 196);
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

    RunState::Accuse
}
