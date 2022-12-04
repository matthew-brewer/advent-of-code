pub mod command;
pub mod input;
pub mod game;

use command::*;
use input::{
    reader::*,
    parser::*
};

use game::{
    moves::*,
    round::*
};

use crate::game::tournament::Tournament;

fn main() {
    let command_arguments = CommandArguments::from_env();

    let reader = Reader::new(&command_arguments.input_file_path);
    let parser = Parser::new(&reader.lines);
    let parsed_input = parser.get_input_pairs();

    println!("Evaluating Rock Paper Scissors challenge based on input file at '{}'", &command_arguments.input_file_path);

    let rounds: Vec<Round> = parsed_input.iter().map(|pair| Round(Move::new_from_input_char(pair.0), Move::new_from_input_char(pair.1))).collect();

    let tournament = Tournament::new(&rounds);
    let tournament_result = tournament.get_result();
    
    println!("Tournament completed!");
    println!("Player A points: {}", tournament_result.player_a_points);
    println!("Player B points: {}", tournament_result.player_b_points);
    println!("Winner: {:?}", tournament_result.winner);
}