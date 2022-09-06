use std::io::stdout;

use crossterm::{
    cursor::{Hide, Show},
    event::{read, Event, KeyCode},
    execute,
};
use state::State;

mod guess;
mod state;
mod words;

fn main() {
    let mut state = State::with_word("RIPED".to_string());
    execute!(stdout(), Hide).unwrap();
    state.render().unwrap();
    while !state.is_finished() {
        match read().unwrap() {
            Event::Key(event) => match event.code {
                KeyCode::Char(chr) => state.handle_input(chr),
                KeyCode::Backspace => state.handle_backspace(),
                KeyCode::Enter => state.handle_return(),
                _ => (),
            },
            _ => (),
        };
        state.render().unwrap();
    }
    execute!(stdout(), Show).unwrap();
}
