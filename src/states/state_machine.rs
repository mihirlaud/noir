use std::collections::LinkedList;

use crate::constants::*;

use wasm_bindgen::{prelude::Closure, JsCast};

use super::{end_state::EndState, main_state::MainState, start_state::StartState, state::State};

pub struct StateMachine {
    stack: LinkedList<Box<dyn State>>,
    start_state: StartState,
    main_state: MainState,
    end_state: EndState,
}

impl StateMachine {
    pub fn new() -> Self {
        Self {
            stack: LinkedList::new(),
            start_state: StartState::new(),
            main_state: MainState::new(),
            end_state: EndState::new(),
        }
    }

    pub fn switch_state(&mut self) {
        unsafe {
            let mut current_state: Box<dyn State> = match game_state {
                CurrentState::Start => Box::new(self.start_state).clone(),
                CurrentState::Main => Box::new(self.main_state).clone(),
                CurrentState::End => Box::new(self.end_state).clone(),
            };

            let window = web_sys::window().unwrap();
            let a = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                let current_state = current_state.as_mut();
                current_state.handle_events(event);

                current_state.draw();

                if current_state.transition().is_some() {
                    let event = web_sys::Event::new(current_state.transition().unwrap().as_str())
                        .expect("error");
                    web_sys::window()
                        .unwrap()
                        .document()
                        .unwrap()
                        .dispatch_event(&event);
                }
            }) as Box<dyn FnMut(_)>);
            window.set_onkeydown(Some(a.as_ref().unchecked_ref()));
            a.forget();

            // let document = window.document().unwrap();

            // let a = Closure::wrap(Box::new(move |event: web_sys::Event| {
            //     //game_loop();
            // }) as Box<dyn FnMut(_)>);
            // document
            //     .add_event_listener_with_callback("transition-main", a.as_ref().unchecked_ref());
            // a.forget();

            // let a = Closure::wrap(Box::new(move |event: web_sys::Event| {
            //     //game_loop();
            // }) as Box<dyn FnMut(_)>);
            // document
            //     .add_event_listener_with_callback("transition-exit", a.as_ref().unchecked_ref());
            // a.forget();
        }
    }
}
