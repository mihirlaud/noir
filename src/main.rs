mod constants;
mod game;
mod states;

use macroquad::prelude::*;
use std::collections::LinkedList;

use constants::*;
use states::start_state::*;
use states::state::*;

#[macroquad::main("Noir")]
async fn main() {

    let font = load_ttf_font("square.ttf").await.unwrap();

    let mut now = get_time();

    let mut stack: LinkedList<Box<dyn State>> = LinkedList::new();
    let start = StartState::new(font.clone());
    stack.push_back(Box::new(start));

    loop {
        let current_state = stack.back_mut().unwrap().as_mut();

        if get_time() - now > FPS_DELAY {
            now = get_time();
            current_state.handle_events();
            current_state.tick();
        }

        current_state.draw();

        if current_state.transition().is_some() {
            let next_state = current_state.transition().unwrap();
            stack.push_back(next_state);
        }

        next_frame().await
    }
}
