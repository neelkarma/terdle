use std::{array::IntoIter, iter::Zip, str::Chars};

use crossterm::style::Color;

#[derive(Debug, Clone, Copy)]
pub enum GuessResult {
    Default,
    NotPresent,
    WrongPosition,
    Correct,
}

impl GuessResult {
    pub fn to_color(&self) -> Color {
        match self {
            Self::Default => Color::Grey,
            Self::NotPresent => Color::DarkGrey,
            Self::WrongPosition => Color::Yellow,
            Self::Correct => Color::Green,
        }
    }
}

#[derive(Debug)]
pub struct Guess {
    pub word: String,
    pub results: [GuessResult; 5],
}

impl Guess {
    pub fn new(guess: String, answer: &str) -> Self {
        let word = guess;
        let mut results = [GuessResult::NotPresent; 5];
        for (idx, chrs) in word.chars().zip(answer.chars()).enumerate() {
            let (gchr, achr) = chrs;
            if achr == gchr {
                results[idx] = GuessResult::Correct;
            } else if answer.contains(gchr) {
                results[idx] = GuessResult::WrongPosition;
            };
        }
        Self { word, results }
    }

    pub fn iter(&self) -> Zip<Chars, IntoIter<GuessResult, 5>> {
        self.word.chars().zip(self.results)
    }
}
