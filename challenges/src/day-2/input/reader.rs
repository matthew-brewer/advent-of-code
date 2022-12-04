use std::fs::File;
use std::io::{
    prelude::*,
    BufReader
};

/// Responsible for opening and reading the input file used to deliver the data for the challenge.
pub struct Reader {
    /// A collection of the lines read from the input data.
    pub lines: Vec<String>
}

impl Reader {
    /// Instantiates a new [Reader] instance based on the contents within the file referenced through the `input_file_path` argument.
    pub fn new(input_file_path: &str) -> Self {
        // Open the file referenced thruogh the `input_file_path` argument and panic if the file cannot be opened
        let file = File::open(&input_file_path).expect(format!("Unable to open the provided input file at {}", &input_file_path).as_str());

        // Create a buffer to read the contents of the file
        let buffer = BufReader::new(file);

        // Iterate through the buffer on a line-by-line basis and confirm that each line can be read
        let lines: Vec<String> = buffer.lines()
            .map(|line| line.expect("An error occurred while attempting to read a line in the input file"))
            .collect();

        // Return a new instance based on the lines read from the file
        Self {
            lines
        }
    }
}