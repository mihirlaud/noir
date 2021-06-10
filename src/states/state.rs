pub trait State {
    fn draw(&self);

    fn handle_events(&mut self);

    fn tick(&mut self);

    fn transition(&self) -> Option<Box<dyn State>>;
}
