use crate::card::Card;

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Hand {
        Hand { cards: Vec::new() }
    }

    //TODO: Sort
}

impl Default for Hand {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for card in &self.cards {
            write!(f, "{}", card)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Rank, Suit};

    #[test]
    fn get_new_hand() {
        let hand = Hand::new();
        assert_eq!(hand.cards.len(), 0);
    }

    #[test]
    fn display_hand_in_string() {
        let mut hand = Hand::new();
        hand.cards.push(Card::new(Suit::Club, Rank::King));
        hand.cards.push(Card::new(Suit::Diamond, Rank::Deuce));
        hand.cards.push(Card::new(Suit::Heart, Rank::Jack));
        assert_eq!(hand.to_string(), "Kc2dJh");
    }
}
