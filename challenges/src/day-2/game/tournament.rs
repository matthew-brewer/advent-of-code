use super::{round::Round};

pub struct Tournament<'a> {
    pub rounds: &'a [Round]
}

impl<'a> Tournament<'a> {
    pub fn new(rounds: &'a [Round]) -> Self {
        Tournament { rounds }
    }

    pub fn get_player_a_points(&self) -> i32 {
        self.rounds.iter().map(|round| round.get_player_a_points()).sum()
    }

    pub fn get_player_b_points(&self) -> i32 {
        self.rounds.iter().map(|round| round.get_player_b_points()).sum()
    }

    pub fn get_tournament_winner(&self, a: i32, b: i32) -> TournamentWinner {
        if a > b {
            return TournamentWinner::PlayerA
        }

        if a < b {
            return TournamentWinner::PlayerB
        }

        TournamentWinner::Draw
    }

    pub fn get_result(&self) -> TournamentResult {
        let player_a_points = self.get_player_a_points();
        let player_b_points = self.get_player_b_points();

        TournamentResult {
            player_a_points,
            player_b_points,
            winner: self.get_tournament_winner(player_a_points, player_b_points)
        }
    }
}

pub struct TournamentResult {
    pub player_a_points: i32,
    pub player_b_points: i32,
    pub winner: TournamentWinner
}

#[derive(Debug)]
pub enum TournamentWinner {
    PlayerA,
    PlayerB,
    Draw
}