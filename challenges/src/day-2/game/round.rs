use super::moves::Move;

#[derive(Debug)]
pub struct Round(pub Move, pub Move);

impl Round {
    pub fn get_result(&self) -> RoundResult {
        let player_a_move = &self.0;
        let player_b_move = &self.1;

        if player_a_move == player_b_move {
            return RoundResult::Draw;
        }

        if player_a_move == &Move::Rock && player_b_move == &Move::Scissors {
            return RoundResult::PlayerA;
        }

        if player_a_move == &Move::Paper && player_b_move == &Move::Rock {
            return RoundResult::PlayerA;
        }

        if player_a_move == &Move::Scissors && player_b_move == &Move::Paper {
            return RoundResult::PlayerA;
        }

        RoundResult::PlayerB
    }

    pub fn get_player_a_points(&self) -> i32 {
        let player_move = &self.0;
        let player_move_points = player_move.get_points_value();

        let result = self.get_result();

        match result {
            RoundResult::PlayerA => player_move_points + 6,
            RoundResult::PlayerB => player_move_points,
            RoundResult::Draw => player_move_points + 3,
        }
    }

    pub fn get_player_b_points(&self) -> i32 {
        let player_move = &self.1;
        let player_move_points = player_move.get_points_value();

        let result = self.get_result();

        match result {
            RoundResult::PlayerA => player_move_points,
            RoundResult::PlayerB => player_move_points + 6,
            RoundResult::Draw => player_move_points + 3
        }
    }
}

pub enum RoundResult {
    PlayerA,
    PlayerB,
    Draw
}