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

impl From<Vec<Card>> for Hand {
    fn from(cards: Vec<Card>) -> Self {
        Hand { cards }
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

    #[test]
    fn from_vec_to_hand() {
        use crate::deck::Deck;
        let deck = Deck::new();
        let hand = Hand::from(deck.cards);
        assert_eq!(hand.cards.len(), 52);
        assert_eq!(hand.to_string(), "2c3c4c5c6c7c8c9cTcJcQcKcAc2d3d4d5d6d7d8d9dTdJdQdKdAd2h3h4h5h6h7h8h9hThJhQhKhAh2s3s4s5s6s7s8s9sTsJsQsKsAs");
    }
}
