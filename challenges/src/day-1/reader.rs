use std::{
    fs::File, 
    io::{
        prelude::*,
        BufReader
    }
};

/// The interface representing the module responsible for reading the contents of the input file used to deliver the data for the challenge.
pub struct ContentsReader<'a> {
    /// Represents the opened file used to deliver the data for the challenge.
    file: &'a File
}

impl<'a> ContentsReader<'a> {
    /// Creates a new [ContentsReader] based on the provided file.
    pub fn new(file: &'a File) -> Self {
        ContentsReader {
            file
        }
    }

    /// Reads each line of the subject [File] into a collection.
    fn read_lines(&self) -> Vec<String> {
        let input_file_buffer = BufReader::new(self.file);
        input_file_buffer.lines()
            .map(|line| line.expect("Could not parse line"))
            .collect()
    }

    /// Parses the contents of the subject [File] into a collection of collections, each representing groups of values that are separated by empty
    /// lines in the source file.
    fn parse_line_groups(&self, lines: &[String]) -> Vec<Vec<String>> {
        let mut groups: Vec<Vec<String>> = vec![];
        let mut current_group_vector: Vec<String> = Vec::new();

        for line in lines.iter() {
            if line.is_empty() {
                groups.push(current_group_vector);
                current_group_vector = Vec::new();
            }
            else {
                current_group_vector.push(line.to_owned());
            }
        }

        groups
    }

    /// Parses the [String] contents of an individual line into an [i32] value.
    fn parse_line_to_int(&self, line: &str) -> i32 {
        line.parse::<i32>().expect(format!("Could not parse string '{}' into a 32-bit integer", line).as_str())
    }

    /// Parses the [String] contents of an individual group of [String] lines into a collection of [i32] values.
    fn parse_group_to_int(&self, group: &Vec<String>) -> Vec<i32> {
        group.iter().map(|line| self.parse_line_to_int(line)).collect()
    }

    /// Parses the [String] contents of the provided collection of groups into a collection of groups of [i32] values instead.
    fn parse_groups_to_int(&self, groups: &Vec<Vec<String>>) -> Vec<Vec<i32>> {
        groups.iter().map(|group| self.parse_group_to_int(group)).collect()
    }

    /// Returns a [Contents] instance whose contents describe the parsed contents of the provided [File] input.
    pub fn to_contents(&self) -> Contents {
        let lines = self.read_lines();
        let line_groups = self.parse_line_groups(&lines);
        let parsed_line_groups = self.parse_groups_to_int(&line_groups);

        Contents {
            value_groups: parsed_line_groups
        }
    }
}

/// Represents the parsed contents of an input file used to deliver the data for the challenge.
pub struct Contents {
    /// A collection of collections that each describe distinct groups within the input data.
    pub value_groups: Vec<Vec<i32>>
}