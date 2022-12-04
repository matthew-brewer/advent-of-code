#[derive(Debug, PartialEq)]
pub enum Move {
    Rock,
    Paper,
    Scissors
}

impl Move {
    pub fn new_from_input_char(input_char: char) -> Self {
        match input_char {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!("Attempt to instantiate Move instance using invalid input character encoding '{}'", input_char)
        }
    }

    pub fn get_points_value(&self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3
        }
    }
}