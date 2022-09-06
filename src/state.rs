use std::{
    collections::BTreeMap,
    io::{stdout, Write},
};

use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Print, Stylize},
    terminal::{Clear, ClearType},
};
use rand::{seq::SliceRandom, thread_rng};

use crate::{
    guess::{Guess, GuessResult},
    words::WORDS,
};

pub struct State {
    pub answer: String,
    pub guesses: Vec<Guess>,
    pub letters: BTreeMap<char, GuessResult>,
    pub input: String,
}

impl State {
    pub fn new() -> Self {
        let answer = WORDS.choose(&mut thread_rng()).unwrap().to_string();
        return Self::with_word(answer);
    }

    pub fn with_word(answer: String) -> Self {
        let answer = answer.to_ascii_uppercase();
        let guesses = Vec::new();
        let input = String::new();

        let mut letters = BTreeMap::new();
        for code in b'A'..=b'Z' {
            let chr = code as char;
            letters.insert(chr, GuessResult::Default);
        }

        Self {
            answer,
            guesses,
            input,
            letters,
        }
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
        if self.input.len() != 5 || !WORDS.contains(&self.input.as_ref()) {
            return;
        };
        self.guesses
            .push(Guess::new(self.input.clone(), &self.answer));

        for (chr, res) in self.guesses.last().unwrap().iter() {
            *self.letters.get_mut(&chr).unwrap() = res;
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
    }

    pub fn render(&self) -> crossterm::Result<()> {
        let mut stdout = stdout();
        queue!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;
        for guess in &self.guesses {
            for (chr, res) in guess.iter() {
                queue!(stdout, Print(chr.with(res.to_color())))?;
            }
            queue!(stdout, Print("\n"))?;
        }
        queue!(stdout, Print(&self.input))?;
        if self.input.len() < 5 && self.guesses.len() < 6 {
            queue!(stdout, Print("_".dark_grey()))?;
        };
        if self.guesses.len() < 6 {
            queue!(stdout, Print("\n"))?;
        }
        queue!(stdout, Print("\n"))?;
        for (chr, res) in &self.letters {
            queue!(stdout, Print(format!("{}", chr).with(res.to_color())))?;
        }

        if self.is_finished() {
            let num_guesses = self.guesses.len();
            if self.guesses.last().unwrap().word == self.answer {
                if num_guesses == 1 {
                    queue!(stdout, Print("\n\nGuessed in 1 try"))?;
                } else {
                    queue!(
                        stdout,
                        Print(format!("\n\nGuessed in {} tries", num_guesses))
                    )?;
                };
            } else {
                queue!(
                    stdout,
                    Print(format!("\n\nThe correct word was {}", self.answer))
                )?;
            };
        };
        stdout.flush()?;
        Ok(())
    }
}
