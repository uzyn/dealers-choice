use crate::error::Error;
use crate::hand::Hand;

pub trait Evaluation {
    fn eval_hand(hand: &Hand) -> Result<u128, Error>;

    fn compare_hands(hand1: &Hand, hand2: &Hand) -> std::cmp::Ordering {
        let score1 = Self::eval_hand(hand1);
        let score2 = Self::eval_hand(hand2);
        score1.cmp(&score2)
    }
}

pub mod highcard;
pub mod lowball_ato5;
