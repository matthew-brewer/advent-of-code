use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader}
};

fn get_input_file() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    let input_file_path = &args.get(2).expect("Missing input file path");
    println!("Received input file path: {}", input_file_path);

    let input_file = File::open(input_file_path).expect("Could not open the file at the provided input file path");

    let input_file_buffer = BufReader::new(input_file);
    input_file_buffer.lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}

fn get_groups(lines: &Vec<String>) -> Vec<Vec<i32>> {
    let mut groups: Vec<Vec<i32>> = vec![];
    let mut current_group_vector: Vec<i32> = Vec::new();

    for line in lines.iter() {
        if line.is_empty() {
            groups.push(current_group_vector);
            current_group_vector = Vec::new();
        }
        else {
            current_group_vector.push(line.parse::<i32>().expect(format!("Could not parse string value '{}' into integer", line).as_str()));
        }
    }

    groups
}

fn get_highest_group_sum(groups: &Vec<Vec<i32>>) -> i32 {    
    groups.iter()
        .map(|group| group.iter().sum())
        .max().unwrap_or(0)
}

fn main() {
    let input = get_input_file();
    let groups = get_groups(&input);
    let highest_value = get_highest_group_sum(&groups);
    
    println!("The highest group value is: {}", highest_value);
}