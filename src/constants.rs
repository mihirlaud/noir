pub enum CurrentState {
    Start,
    End,
    Main,
}

pub static mut game_state: CurrentState = CurrentState::Start;
