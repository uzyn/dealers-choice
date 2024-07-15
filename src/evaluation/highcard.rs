use super::*;

pub struct Highcard {}

impl Evaluation for Highcard {
    fn eval_hand(hand: &Hand) -> Result<u32, Error> {
        if hand.cards.len() > 1 {
            return Err(Error::InvalidHand);
        }
        Ok(hand.cards[0].rank as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Rank, Suit};
    use crate::hand::Hand;

    // test eval_hand
    #[test]
    fn test_eval_hand_valid() {
        let mut hand = Hand::new();
        hand.cards.push(Card::new(Suit::Diamond, Rank::King));
        assert_eq!(Highcard::eval_hand(&hand), Ok(11));
        assert_eq!(Highcard::eval_hand(&Hand::from("Ad".to_string())), Ok(12));
        assert_eq!(Highcard::eval_hand(&Hand::from("2h".to_string())), Ok(0));
        assert_eq!(Highcard::eval_hand(&Hand::from("2c".to_string())), Ok(0));
    }

    #[test]
    fn test_eval_hand_invalid() {
        let mut hand = Hand::new();
        hand.cards.push(Card::new(Suit::Diamond, Rank::King));
        hand.cards.push(Card::new(Suit::Heart, Rank::Queen));
        let result = Highcard::eval_hand(&hand);
        assert_eq!(result, Err(Error::InvalidHand));
    }
}
