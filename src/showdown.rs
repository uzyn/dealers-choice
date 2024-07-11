use crate::*;

pub struct Showdown<'a> {
    rule: &'a dyn rule::Rule,
    players: Vec<hand::Hand>,
    board: Option<hand::Hand>,
}
