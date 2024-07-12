use crate::rule::Rule;
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

    pub fn determine_payouts<R: Rule>(&self) -> Payouts {
        R::determine_payouts(&self.players, self.board.as_ref())
    }
}
