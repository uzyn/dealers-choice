use crate::error::Error;
use crate::hand::Hand;

pub mod highcard;
pub mod lowball_ato5;

pub trait EvalHand: Default {
    fn eval_hand(hand: &Hand) -> Result<u128, Error>;

    fn compare_hands(hand1: &Hand, hand2: &Hand) -> std::cmp::Ordering {
        let score1 = Self::eval_hand(hand1);
        let score2 = Self::eval_hand(hand2);
        score1.cmp(&score2)
    }
}

#[derive(Debug)]
pub struct Evaluation<T>
where
    T: EvalHand,
{
    pub hand: Hand,
    pub evaluator: T,
    pub score: u128,
}

impl<T> TryFrom<Hand> for Evaluation<T>
where
    T: EvalHand,
{
    type Error = Error;

    fn try_from(hand: Hand) -> Result<Self, Error> {
        Ok(Self {
            hand: hand.clone(),
            evaluator: T::default(),
            score: T::eval_hand(&hand)?,
        })
    }
}

impl<T> TryFrom<&str> for Evaluation<T>
where
    T: EvalHand,
{
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Error> {
        let hand = Hand::try_from(s)?;
        Self::try_from(hand)
    }
}

impl<T> PartialEq for Evaluation<T>
where
    T: EvalHand,
{
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl<T> Eq for Evaluation<T> where T: EvalHand {}

impl<T> Ord for Evaluation<T>
where
    T: EvalHand,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl<T> PartialOrd for Evaluation<T>
where
    T: EvalHand,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
