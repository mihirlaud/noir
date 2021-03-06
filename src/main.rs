mod components;
mod constants;
mod gui;
mod map;
mod player;
mod story;

use std::collections::BTreeMap;

use components::*;
use constants::*;
use gui::{
    draw_accuse_panel, draw_examination_panel, draw_log, draw_notes, draw_sidebar, draw_talk_panel,
    view_log, GameOverResult, MainMenuSelection, PauseMenuSelection,
};
use rltk::{Console, GameState, Rltk, RGB};
use specs::prelude::*;

use crate::{
    gui::{Log, NoteBoxes, Options, Time},
    map::Map,
    story::{Clue, PlayerNotes, Story, Suspect},
};

#[derive(PartialEq, Clone, Copy)]
pub enum RunState {
    MainMenu { selection: MainMenuSelection },
    AwaitingInput,
    Talking,
    Examining,
    Notes,
    Accuse,
    Log { page: usize },
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
        let mut examination_checker = ExaminationChecker {};
        examination_checker.run_now(&self.ecs);
        let mut movement_checker = MovementChecker {};
        movement_checker.run_now(&self.ecs);
        self.ecs.maintain();
    }

    fn new_game(&mut self) {
        self.ecs = World::new();

        self.ecs.register::<Position>();
        self.ecs.register::<Renderable>();
        self.ecs.register::<ConversationAI>();
        self.ecs.register::<Suspect>();
        self.ecs.register::<Clue>();
        self.ecs.register::<MovementAI>();

        self.ecs.insert(RunState::MainMenu {
            selection: MainMenuSelection::Play,
        });

        self.ecs.insert(PlayerPosition { x: 1, y: 1 });
        self.ecs.insert(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::WHITE),
            bg: RGB::named(rltk::BLACK),
        });

        let story = Story::gen_rand();
        let map = Map::from_story(&story, self);

        self.ecs.insert(story);
        self.ecs.insert(map);

        let note_boxes = NoteBoxes::new();
        self.ecs.insert(note_boxes);

        let time = Time::new();

        let mut log = Log { log: vec![] };
        log.log_message(
            &time,
            "Game",
            "Hello Detective. Welcome to Noir!",
            RGB::named(rltk::WHITE),
        );
        log.log_message(
            &time,
            "Game",
            "Use arrow keys to move around.",
            RGB::named(rltk::HOTPINK),
        );
        self.ecs.insert(log);

        let notes = PlayerNotes::new();
        self.ecs.insert(notes);

        self.ecs.insert(time);

        let talk = TalkEntity {
            entity: self.ecs.create_entity().build(),
        };
        self.ecs.insert(talk);

        let exam = ExamEntity {
            entity: self.ecs.create_entity().build(),
        };
        self.ecs.insert(exam);

        let mut options = BTreeMap::new();
        options.insert('P', "Pause".to_string());
        options.insert('N', "Notes".to_string());
        options.insert('L', "View Log".to_string());

        let options = Options { options };
        self.ecs.insert(options);
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
            RunState::Accuse { .. } => {}
            RunState::Talking => {
                draw_log(self, ctx);
                draw_sidebar(self, ctx);
            }
            RunState::Examining => {
                draw_log(self, ctx);
                draw_sidebar(self, ctx);
            }
            RunState::Notes => {
                draw_log(self, ctx);
                draw_sidebar(self, ctx);
            }
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
                        MainMenuSelection::Play => {
                            newrunstate = RunState::AwaitingInput;
                        }
                        MainMenuSelection::Quit => {
                            newrunstate = RunState::GameOver {
                                result: GameOverResult::None,
                            }
                        }
                    },
                };
            }
            RunState::AwaitingInput => {
                newrunstate = player::input(self, ctx);
            }
            RunState::Talking => {
                newrunstate = draw_talk_panel(self, ctx);
            }
            RunState::Examining => {
                newrunstate = draw_examination_panel(self, ctx);
            }
            RunState::Notes => {
                newrunstate = draw_notes(self, ctx);
            }
            RunState::Accuse => {
                newrunstate = draw_accuse_panel(self, ctx);
            }
            RunState::Log { page } => {
                let result = view_log(self, ctx, page);
                match result {
                    gui::ViewLogResult::None(new_page) => {
                        newrunstate = RunState::Log { page: new_page }
                    }
                    gui::ViewLogResult::Up(new_page) => {
                        newrunstate = RunState::Log { page: new_page }
                    }
                    gui::ViewLogResult::Down(new_page) => {
                        newrunstate = RunState::Log { page: new_page }
                    }
                    gui::ViewLogResult::Exit => newrunstate = RunState::AwaitingInput,
                }
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
                        self.new_game();
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

    gs.new_game();

    rltk::main_loop(context, gs)
}
