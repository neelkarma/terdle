use crate::guess::GuessResult;

pub struct Hints {
    data: [GuessResult; 26],
}

impl Hints {
    pub fn new() -> Self {
        Self {
            data: [GuessResult::Default; 26],
        }
    }

    fn char_to_index(c: char) -> u8 {
        (c as u8) - 65
    }

    fn index_to_char(i: u8) -> char {
        (i + 65) as char
    }

    pub fn set(&mut self, c: char, res: GuessResult) {
        self.data[Self::char_to_index(c) as usize] = res
    }

    pub fn iter(&self) -> Vec<(char, &GuessResult)> {
        self.data
            .iter()
            .enumerate()
            .map(|(i, res)| (Self::index_to_char(i as u8), res))
            .collect()
    }
}
