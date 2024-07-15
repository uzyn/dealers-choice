use super::*;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Razz {}

impl Evaluation for Razz {
    fn evaluate_hand(_hand: &Hand, _board: Option<&Hand>) -> u32 {
        0 // TODO: replace dummy
    }

    fn is_valid(players: &[Hand], board: Option<&Hand>) -> Result<(), EvaluationError> {
        // Check that hand is 5-7 cards and that the numbers are the same for all players.
        let mut players_hand_card_count = 0;
        for player in players {
            if player.cards.len() < 5 || player.cards.len() > 7 {
                return Err(EvaluationError::InvalidHand);
            }
            players_hand_card_count = player.cards.len();
        }
        for player in players {
            if player.cards.len() != players_hand_card_count {
                return Err(EvaluationError::InvalidHand);
            }
        }

        // Also ensure that there are no boards
        if board.is_some() {
            return Err(EvaluationError::InvalidBoard);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Rank, Suit};
    use crate::hand::Hand;

    #[test]
    fn test_valid_hands() {
        let mut hand1 = Hand::new();
        hand1.cards.push(Card::new(Suit::Diamond, Rank::King));
        hand1.cards.push(Card::new(Suit::Heart, Rank::Queen));
        hand1.cards.push(Card::new(Suit::Spade, Rank::Jack));
        hand1.cards.push(Card::new(Suit::Club, Rank::Ten));
        hand1.cards.push(Card::new(Suit::Diamond, Rank::Nine));

        let mut hand2 = Hand::new();
        hand2.cards.push(Card::new(Suit::Club, Rank::Ace));
        hand2.cards.push(Card::new(Suit::Diamond, Rank::Trey));
        hand2.cards.push(Card::new(Suit::Heart, Rank::Five));
        hand2.cards.push(Card::new(Suit::Spade, Rank::Seven));
        hand2.cards.push(Card::new(Suit::Club, Rank::Eight));

        let players = vec![hand1, hand2];
        assert_eq!(Razz::is_valid(&players, None), Ok(()));
    }

    #[test]
    fn test_invalid_hands() {
        let mut hand1 = Hand::new();
        hand1.cards.push(Card::new(Suit::Diamond, Rank::King));
        hand1.cards.push(Card::new(Suit::Heart, Rank::Queen));
        hand1.cards.push(Card::new(Suit::Spade, Rank::Jack));
        hand1.cards.push(Card::new(Suit::Club, Rank::Ten));
        hand1.cards.push(Card::new(Suit::Diamond, Rank::Nine));

        let mut hand2 = Hand::new();
        hand2.cards.push(Card::new(Suit::Club, Rank::Ace));
        hand2.cards.push(Card::new(Suit::Diamond, Rank::Trey));
        hand2.cards.push(Card::new(Suit::Heart, Rank::Five));
        hand2.cards.push(Card::new(Suit::Spade, Rank::Seven));

        let players = vec![hand1, hand2];
        assert_eq!(
            Razz::is_valid(&players, None),
            Err(EvaluationError::InvalidHand)
        );
    }

    #[test]
    fn test_invalid_hands_with_board() {
        let mut hand1 = Hand::new();
        hand1.cards.push(Card::new(Suit::Diamond, Rank::King));
        hand1.cards.push(Card::new(Suit::Heart, Rank::Queen));
        hand1.cards.push(Card::new(Suit::Spade, Rank::Jack));
        hand1.cards.push(Card::new(Suit::Club, Rank::Ten));
        hand1.cards.push(Card::new(Suit::Diamond, Rank::Nine));

        let mut hand2 = Hand::new();
        hand2.cards.push(Card::new(Suit::Club, Rank::Ace));
        hand2.cards.push(Card::new(Suit::Diamond, Rank::Trey));
        hand2.cards.push(Card::new(Suit::Heart, Rank::Five));
        hand2.cards.push(Card::new(Suit::Spade, Rank::Seven));
        hand2.cards.push(Card::new(Suit::Club, Rank::Eight));

        let mut board = Hand::new();
        board.cards.push(Card::new(Suit::Spade, Rank::King));
        board.cards.push(Card::new(Suit::Club, Rank::Queen));
        board.cards.push(Card::new(Suit::Diamond, Rank::Jack));
        board.cards.push(Card::new(Suit::Heart, Rank::Ten));
        board.cards.push(Card::new(Suit::Spade, Rank::Nine));

        let players = vec![hand1, hand2];
        assert_eq!(
            Razz::is_valid(&players, Some(&board)),
            Err(EvaluationError::InvalidBoard)
        );
    }
}
