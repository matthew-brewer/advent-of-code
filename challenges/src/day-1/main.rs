pub mod arguments;
pub mod reader;
pub mod evaluator;

use std::fs;

use arguments::*;
use reader::*;
use evaluator::*;

fn main() {
    // Parse and organize the arguments passed at the instantiation of the process
    let arguments = Arguments::from_env();

    // Open the input file referenced in the arguments passed to the process
    let file = fs::File::open(&arguments.input_file_path).expect(format!("An error occurred while attempting to open the input file at location '{}'", &arguments.input_file_path).as_str());
    
    // Read the contents of the input file referenced in the arguments passed to the process
    let input_contents = ContentsReader::new(&file).to_contents();

    // Evaluate the contents of the provided input file according to the specifications of the challenge
    let evaluator = Evaluator::new(&input_contents);
    
    println!("The highest group value is: {}", &evaluator.get_sorted_group_sum(1));
    println!("The cumulative value among the highest three (3) groups is: {}", &evaluator.get_sorted_group_sum(3));
}