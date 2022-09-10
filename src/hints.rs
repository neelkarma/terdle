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

    fn char_to_index(c: char) -> usize {
        match c {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            'D' => 3,
            'E' => 4,
            'F' => 5,
            'G' => 6,
            'H' => 7,
            'I' => 8,
            'J' => 9,
            'K' => 10,
            'L' => 11,
            'M' => 12,
            'N' => 13,
            'O' => 14,
            'P' => 15,
            'Q' => 16,
            'R' => 17,
            'S' => 18,
            'T' => 19,
            'U' => 20,
            'V' => 21,
            'W' => 22,
            'X' => 23,
            'Y' => 24,
            'Z' => 25,
            _ => panic!("Character isn't an ascii uppercase letter."),
        }
    }

    fn index_to_char(index: usize) -> char {
        match index {
            0 => 'A',
            1 => 'B',
            2 => 'C',
            3 => 'D',
            4 => 'E',
            5 => 'F',
            6 => 'G',
            7 => 'H',
            8 => 'I',
            9 => 'J',
            10 => 'K',
            11 => 'L',
            12 => 'M',
            13 => 'N',
            14 => 'O',
            15 => 'P',
            16 => 'Q',
            17 => 'R',
            18 => 'S',
            19 => 'T',
            20 => 'U',
            21 => 'V',
            22 => 'W',
            23 => 'X',
            24 => 'Y',
            25 => 'Z',
            _ => panic!("Invalid index"),
        }
    }

    pub fn set(&mut self, c: char, res: GuessResult) {
        self.data[Self::char_to_index(c)] = res
    }

    pub fn iter(&self) -> Vec<(char, &GuessResult)> {
        self.data
            .iter()
            .enumerate()
            .map(|(index, res)| (Self::index_to_char(index), res))
            .collect()
    }
}
