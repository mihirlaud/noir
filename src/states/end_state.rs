use crate::constants::FONT_SIZE;

use super::state::State;
use macroquad::prelude::*;

pub struct EndState {
    font: Font,
}

impl EndState {
    pub fn new(font: Font) -> Self {
        Self { font }
    }
}

impl State for EndState {
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
            "THANK YOU FOR PLAYING",
            screen_width() / 2.0 - 21.0 * FONT_SIZE / 2.0,
            screen_height() / 2.0 - 1.0 * FONT_SIZE / 2.0,
            params,
        );
    }

    fn handle_events(&mut self) {}

    fn tick(&mut self) {}

    fn transition(&self) -> Option<Box<dyn State>> {
        None
    }
}
