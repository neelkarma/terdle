use std::io::stdout;

use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Print, Stylize},
};
use rand::{seq::SliceRandom, thread_rng};

use crate::{
    guess::Guess,
    hints::Hints,
    words::{OTHER_WORDS, START_WORDS},
};

pub struct State {
    pub answer: String,
    pub guesses: Vec<Guess>,
    pub hints: Hints,
    pub input: String,
    pub exited: bool,
}

impl State {
    pub fn new() -> Self {
        let answer = START_WORDS.choose(&mut thread_rng()).unwrap().to_string();
        return Self::with_word(answer);
    }

    pub fn with_word(answer: String) -> Self {
        let answer = answer.to_ascii_uppercase();
        let guesses = Vec::new();
        let input = String::new();
        let hints = Hints::new();
        let exited = false;

        Self {
            answer,
            guesses,
            input,
            hints,
            exited,
        }
    }

    pub fn handle_exit(&mut self) {
        self.exited = true;
    }

    pub fn handle_input(&mut self, chr: char) {
        if !chr.is_ascii_alphabetic() || self.input.len() >= 5 {
            return;
        };
        self.input.push(chr.to_ascii_uppercase());
    }

    pub fn handle_backspace(&mut self) {
        self.input.pop();
    }

    pub fn handle_return(&mut self) {
        if self.input.len() != 5
            || !(START_WORDS.contains(&self.input.as_ref())
                || OTHER_WORDS.contains(&self.input.as_ref()))
        {
            return;
        };
        self.guesses
            .push(Guess::new(self.input.clone(), &self.answer));

        for (chr, res) in self.guesses.last().unwrap().iter() {
            self.hints.set(chr, res);
        }

        self.input = String::new();
    }

    pub fn is_finished(&self) -> bool {
        self.guesses
            .iter()
            .map(|guess| &guess.word)
            .collect::<Vec<_>>()
            .contains(&&self.answer)
            || self.guesses.len() >= 6
            || self.exited
    }

    pub fn render(&self) -> crossterm::Result<()> {
        let mut out = String::new();

        // Previous guesses
        for guess in &self.guesses {
            for (chr, res) in guess.iter() {
                out.push_str(&chr.with(res.to_color()).to_string());
            }
            out.push('\n');
        }

        // Current user input
        out.push_str(&self.input);
        if self.input.len() < 5 && self.guesses.len() < 6 {
            out.push_str(&"_".dark_grey().to_string());
        }
        out.push_str(&" ".repeat(5 - self.input.len()));
        if self.guesses.len() < 6 {
            out.push('\n');
        }
        out.push_str(&" ".repeat(26));
        out.push('\n');

        // Hints
        for (chr, res) in &self.hints.iter() {
            out.push_str(&chr.with(res.to_color()).to_string());
        }

        // Finished text
        if self.is_finished() {
            out.push_str("\n\n");
            let num_guesses = self.guesses.len();
            if self.exited {
                out.push_str(&format!("The correct word was {}", &self.answer));
            } else if self.guesses.last().unwrap().word == self.answer {
                if num_guesses == 1 {
                    out.push_str("Guessed in 1 try");
                } else {
                    out.push_str(&format!("Guessed in {} tries", num_guesses));
                };
            } else {
                out.push_str(&format!("The correct word was {}", &self.answer));
            };
        }

        execute!(stdout(), MoveTo(0, 0), Print(out))?;
        Ok(())
    }
}
