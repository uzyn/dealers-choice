use crate::hand::Hand;
use crate::showdown::Payouts;

pub mod razz;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum RuleError {
    InvalidHand,
    InvalidBoard,
}

pub trait Rule {
    fn evaluate_hand(hand: &Hand, board: Option<&Hand>) -> u32;

    fn compare_hands(hand1: &Hand, hand2: &Hand, board: Option<&Hand>) -> std::cmp::Ordering {
        let score1 = Self::evaluate_hand(hand1, board);
        let score2 = Self::evaluate_hand(hand2, board);
        score1.cmp(&score2)
    }

    // Insert custom rule validity check here, e.g. ensuring that there are 2 cards per player and 5 community cards for hold'em.
    fn is_valid(_players: &[Hand], _board: Option<&Hand>) -> Result<(), RuleError> {
        Ok(())
    }

    // Returns a vector of the same size as players with the percentage of pot won
    // If it's a tie, and assuming 2 players, returned value would be vec!<0.5, 0.5>
    fn determine_payouts(players: &[Hand], board: Option<&Hand>) -> Result<Payouts, RuleError> {
        Self::is_valid(players, board)?;

        // Default logic for single-pot games, e.g. not hi-lo.

        let mut winner = 0;
        for i in 1..players.len() {
            if Self::compare_hands(&players[i], &players[winner], board)
                == std::cmp::Ordering::Greater
            {
                winner = i;
            }
        }

        // Check for ties
        let mut winners = vec![false; players.len()];
        let mut winner_count = 0;
        for i in 0..players.len() {
            if i == winner
                || Self::compare_hands(&players[i], &players[winner], board)
                    == std::cmp::Ordering::Equal
            {
                winner_count += 1;
                winners[i] = true;
            }
        }

        let mut payouts: Payouts = vec![0.0; players.len()];
        let pot_split: f32 = 1.0 / winner_count as f32;
        for i in 0..players.len() {
            if winners[i] {
                payouts[i] = pot_split;
            }
        }

        Ok(payouts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Rank, Suit};
    use crate::hand::Hand;

    struct MockRule {}

    impl Rule for MockRule {
        fn evaluate_hand(hand: &Hand, _board: Option<&Hand>) -> u32 {
            hand.cards[0].rank as u32
        }
    }

    #[test]
    fn test_compare_hands() {
        let mut hand1 = Hand::new();
        hand1.cards.push(Card::new(Suit::Diamond, Rank::King));

        let mut hand2 = Hand::new();
        hand2.cards.push(Card::new(Suit::Heart, Rank::Queen));

        let mut hand3 = Hand::new();
        hand3.cards.push(Card::new(Suit::Spade, Rank::Queen));

        assert_eq!(
            MockRule::compare_hands(&hand2, &hand3, None),
            std::cmp::Ordering::Equal
        );
        assert_eq!(
            MockRule::compare_hands(&hand2, &hand2, None),
            std::cmp::Ordering::Equal
        ); // same hand
        assert_eq!(
            MockRule::compare_hands(&hand1, &hand2, None),
            std::cmp::Ordering::Greater
        );
        assert_eq!(
            MockRule::compare_hands(&hand2, &hand1, None),
            std::cmp::Ordering::Less
        );
    }

    #[test]
    fn test_determine_payouts() {
        let mut hand1 = Hand::new();
        hand1.cards.push(Card::new(Suit::Diamond, Rank::King));

        let mut hand2 = Hand::new();
        hand2.cards.push(Card::new(Suit::Heart, Rank::Queen));

        let mut hand3 = Hand::new();
        hand3.cards.push(Card::new(Suit::Heart, Rank::Trey));

        let players = vec![hand1, hand2, hand3];
        let payouts = MockRule::determine_payouts(&players, None);
        assert_eq!(payouts, Ok(vec![1.0, 0.0, 0.0]));
    }

    #[test]
    fn test_split_payouts() {
        let mut hand1 = Hand::new();
        hand1.cards.push(Card::new(Suit::Diamond, Rank::Four));

        let mut hand2 = Hand::new();
        hand2.cards.push(Card::new(Suit::Heart, Rank::Queen));

        let mut hand3 = Hand::new();
        hand3.cards.push(Card::new(Suit::Club, Rank::Queen));

        let mut hand4 = Hand::new();
        hand4.cards.push(Card::new(Suit::Spade, Rank::Queen));

        let players = vec![hand1, hand2, hand3, hand4];
        let payouts = MockRule::determine_payouts(&players, None);
        assert_eq!(payouts, Ok(vec![0.0, 1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0]));
    }
}
