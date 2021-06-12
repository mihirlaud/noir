mod states;

use states::end_state::EndState;
use states::main_state::MainState;
use states::start_state::StartState;
use states::state::State;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Event;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
pub fn start_game() {
    let mut start_state = StartState::new();
    start_state.draw();

    let window = web_sys::window().unwrap();
    let a = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        start_state.handle_events(event);

        start_state.draw();

        if start_state.transition().is_some() {
            let event =
                web_sys::Event::new(start_state.transition().unwrap().as_str()).expect("error");
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .dispatch_event(&event);
        }
    }) as Box<dyn FnMut(_)>);
    window.set_onkeydown(Some(a.as_ref().unchecked_ref()));
    a.forget();

    let document = window.document().unwrap();

    let a = Closure::wrap(Box::new(move |event: Event| {
        game_loop();
    }) as Box<dyn FnMut(_)>);
    document.add_event_listener_with_callback("transition-main", a.as_ref().unchecked_ref());
    a.forget();

    let a = Closure::wrap(Box::new(move |event: Event| {
        exit();
    }) as Box<dyn FnMut(_)>);
    document.add_event_listener_with_callback("transition-exit", a.as_ref().unchecked_ref());
    a.forget();
}

pub fn game_loop() {
    let mut main_state = MainState::new();
    main_state.draw();

    let window = web_sys::window().unwrap();
    let a = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        main_state.handle_events(event);

        main_state.draw();

        if main_state.transition().is_some() {
            let event =
                web_sys::Event::new(main_state.transition().unwrap().as_str()).expect("error");
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .dispatch_event(&event);
        }
    }) as Box<dyn FnMut(_)>);
    window.set_onkeydown(Some(a.as_ref().unchecked_ref()));
    a.forget();

    let document = window.document().unwrap();
    let a = Closure::wrap(Box::new(move |event: Event| {
        //game_loop();
    }) as Box<dyn FnMut(_)>);
    document.add_event_listener_with_callback("transition-main", a.as_ref().unchecked_ref());
    a.forget();
}

pub fn exit() {
    let mut exit_state = EndState::new();
    exit_state.draw();

    let window = web_sys::window().unwrap();
    let a = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        exit_state.handle_events(event);

        exit_state.draw();

        if exit_state.transition().is_some() {
            let event =
                web_sys::Event::new(exit_state.transition().unwrap().as_str()).expect("error");
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .dispatch_event(&event);
        }
    }) as Box<dyn FnMut(_)>);
    window.set_onkeydown(Some(a.as_ref().unchecked_ref()));
    a.forget();
}

pub fn clear_canvas() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
}
