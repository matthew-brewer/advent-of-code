pub struct Parser<'a> {
    pub lines: &'a [String]
}

pub struct InputPair(pub char, pub char);

impl<'a> Parser<'a> {
    pub fn new(lines: &'a [String]) -> Self {
        Self {
            lines
        }
    }

    fn get_line_input_pair(&self, line: &String) -> InputPair {
        let chars: Vec<char> = line.chars().filter(|char| !char.is_whitespace()).collect();
        InputPair(chars[0], chars[1])
    }

    pub fn get_input_pairs(&self) -> Vec<InputPair> {
        self.lines.iter().map(|line| self.get_line_input_pair(line)).collect::<Vec<InputPair>>()
    }
}