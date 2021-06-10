use crate::constants::FONT_SIZE;

use super::{end_state::EndState, main_state::MainState, state::State};
use macroquad::prelude::*;

pub struct StartState {
    font: Font,
    needs_transition: String,
    selected: i32,
}

impl StartState {
    pub fn new(font: Font) -> Self {
        Self {
            font,
            needs_transition: String::from("none"),
            selected: 1,
        }
    }
}

impl State for StartState {
    fn draw(&self) {
        clear_background(BLACK);
        let params = TextParams {
            font: self.font,
            font_size: FONT_SIZE as u16,
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            color: WHITE,
        };
        draw_text_ex(
            "NOIR",
            screen_width() / 2.0 - 4.0 * FONT_SIZE / 2.0,
            screen_height() / 2.0 - 6.0 * FONT_SIZE / 2.0,
            params,
        );
        draw_text_ex(
            if self.selected == 1 {
                "> PLAY"
            } else {
                "  PLAY"
            },
            screen_width() / 2.0 - 6.0 * FONT_SIZE / 2.0,
            screen_height() / 2.0 - 1.0 * FONT_SIZE / 2.0,
            params,
        );
        draw_text_ex(
            if self.selected == 2 {
                "> EXIT"
            } else {
                "  EXIT"
            },
            screen_width() / 2.0 - 6.0 * FONT_SIZE / 2.0,
            screen_height() / 2.0 + 5.0 * FONT_SIZE / 2.0,
            params,
        );
    }

    fn handle_events(&mut self) {
        if is_key_down(KeyCode::Down) {
            self.selected += 1;
        } else if is_key_down(KeyCode::Up) {
            self.selected -= 1;
        }

        if self.selected < 1 {
            self.selected = 2;
        } else if self.selected > 2 {
            self.selected = 1;
        }

        if is_key_down(KeyCode::Enter) {
            if self.selected == 1 {
                self.needs_transition = String::from("game");
            } else {
                self.needs_transition = String::from("exit");
            }
        }
    }

    fn tick(&mut self) {}

    fn transition(&self) -> Option<Box<dyn State>> {
        match self.needs_transition.as_str() {
            "game" => {
                let main_state = MainState::new(self.font);
                Some(Box::new(main_state))
            }
            "exit" => {
                let end_state = EndState::new(self.font);
                Some(Box::new(end_state))
            }
            _ => None,
        }
    }
}
