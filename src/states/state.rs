use wasm_bindgen::JsValue;

pub trait State {
    fn box_clone(&self) -> Box<State>;

    fn draw(&self) -> Result<(), JsValue>;

    fn handle_events(&mut self, event: web_sys::KeyboardEvent);

    fn transition(&self) -> Option<String>;
}

impl Clone for Box<State> {
    fn clone(&self) -> Box<State> {
        self.box_clone()
    }
}
