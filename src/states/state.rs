use wasm_bindgen::JsValue;

pub trait State {
    fn draw(&self) -> Result<(), JsValue>;

    fn handle_events(&mut self, event: web_sys::KeyboardEvent);

    fn transition(&self) -> Option<String>;
}
