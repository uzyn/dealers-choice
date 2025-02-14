use super::*;

// Simple evaluation algorithm mainly for testing
#[derive(Default, Debug)]
pub struct Highcard {}

impl EvalHand for Highcard {
    fn eval_hand(hand: &Hand) -> Result<u128, Error> {
        if hand.cards.len() != 1 {
            return Err(Error::InvalidHand);
        }
        Ok(hand.cards[0].rank as u128)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Rank, Suit};
    use crate::hand::Hand;

    #[test]
    fn test_eval_hand_valid() {
        let mut hand = Hand::new();
        hand.cards.push(Card::new(Suit::Diamonds, Rank::King));
        assert_eq!(Highcard::eval_hand(&hand), Ok(11));
        assert_eq!(Highcard::eval_hand(&Hand::try_from("Ad").unwrap()), Ok(12));
        assert_eq!(Highcard::eval_hand(&Hand::try_from("2h").unwrap()), Ok(0));
        assert_eq!(Highcard::eval_hand(&Hand::try_from("2c").unwrap()), Ok(0));
    }

    #[test]
    fn test_eval_hand_invalid() {
        let mut hand = Hand::new();
        hand.cards.push(Card::new(Suit::Diamonds, Rank::King));
        hand.cards.push(Card::new(Suit::Hearts, Rank::Queen));
        let result = Highcard::eval_hand(&hand);
        assert_eq!(result, Err(Error::InvalidHand));
    }

    // test compare_hands
    #[test]
    fn test_compare_hands() {
        let mut hand1 = Hand::new();
        hand1.cards.push(Card::new(Suit::Diamonds, Rank::King));

        let mut hand2 = Hand::new();
        hand2.cards.push(Card::new(Suit::Hearts, Rank::Queen));

        let mut hand3 = Hand::new();
        hand3.cards.push(Card::new(Suit::Spades, Rank::Queen));

        assert_eq!(
            Highcard::compare_hands(&hand1, &hand2),
            std::cmp::Ordering::Greater
        );
        assert_eq!(
            Highcard::compare_hands(&hand2, &hand1),
            std::cmp::Ordering::Less
        );
        assert_eq!(
            Highcard::compare_hands(&hand1, &hand1),
            std::cmp::Ordering::Equal
        );
        assert_eq!(
            Highcard::compare_hands(&hand2, &hand3),
            std::cmp::Ordering::Equal
        );
    }

    #[test]
    fn test_direct_comparison() {
        let h_j = Evaluation::<Highcard>::try_from("Jd").unwrap();
        let h_6 = Evaluation::<Highcard>::try_from("6s").unwrap();
        let h_6d = Evaluation::<Highcard>::try_from("6d").unwrap();
        let h_a = Evaluation::<Highcard>::try_from("Ad").unwrap();

        assert!(h_j > h_6);
        assert!(h_6 == h_6d);
        assert!(h_j < h_a);
    }
}
