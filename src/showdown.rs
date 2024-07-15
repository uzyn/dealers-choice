use crate::evaluation::{Evaluation, EvaluationError};

use crate::hand::Hand;

pub type Payouts = Vec<f32>;

pub struct Showdown {
    players: Vec<Hand>,
    board: Option<Hand>,
}
impl Showdown {
    pub fn new(players: Vec<Hand>, board: Option<Hand>) -> Showdown {
        Showdown { players, board }
    }

    pub fn determine_payouts<R: Evaluation>(&self) -> Result<Payouts, EvaluationError> {
        R::determine_payouts(&self.players, self.board.as_ref())
    }
}
