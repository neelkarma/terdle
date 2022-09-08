use std::{io::stdout, process::exit};

use crossterm::{
    cursor::{Hide, Show},
    event::{read, Event, KeyCode, KeyModifiers},
    execute,
    terminal::enable_raw_mode,
};
use state::State;

mod guess;
mod state;
mod words;

fn main() {
    let mut state = State::new();
    execute!(stdout(), Hide).unwrap();
    enable_raw_mode().unwrap();
    state.render().unwrap();
    while !state.is_finished() {
        match read().unwrap() {
            Event::Key(event) => {
                if event.code == KeyCode::Char('c')
                    && event.modifiers.contains(KeyModifiers::CONTROL)
                {
                    exit(0);
                };

                match event.code {
                    KeyCode::Char(chr) => state.handle_input(chr),
                    KeyCode::Backspace => state.handle_backspace(),
                    KeyCode::Enter => state.handle_return(),
                    _ => (),
                };
            }
            _ => (),
        };
        state.render().unwrap();
    }
    execute!(stdout(), Show).unwrap();
}
