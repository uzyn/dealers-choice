use crate::card::Card;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Hand {
        Hand { cards: Vec::new() }
    }

    pub fn new_sorted_by_rank(&self) -> Hand {
        // Make a copy of Hand, sort and return the copy
        let mut hand = self.clone();
        hand.cards.sort_by(|a, b| a.rank.cmp(&b.rank));
        hand
    }

    // pub fn sort_by_suit(&self) -> Hand {
    //     self.cards.sort_by(|a, b| a.suit.cmp(&b.suit))
    // }
}

impl Default for Hand {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for card in &self.cards[..self.cards.len() - 1] {
            write!(f, "{} ", card)?;
        }
        if let Some(last_card) = self.cards.last() {
            write!(f, "{}", last_card)?;
        }
        Ok(())
    }
}

impl From<Vec<Card>> for Hand {
    fn from(cards: Vec<Card>) -> Hand {
        Hand { cards }
    }
}

impl From<String> for Hand {
    fn from(s: String) -> Hand {
        let mut cards: Vec<Card> = Vec::new();
        for card_str in s.split_whitespace() {
            cards.push(Card::from(card_str.to_string()));
        }
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
        assert_eq!(hand.to_string(), "Kc 2d Jh");
    }

    #[test]
    fn from_vec_to_hand() {
        use crate::deck::Deck;
        let deck = Deck::new();
        let hand = Hand::from(deck.cards);
        assert_eq!(hand.cards.len(), 52);
        assert_eq!(hand.to_string(), "2c 3c 4c 5c 6c 7c 8c 9c Tc Jc Qc Kc Ac 2d 3d 4d 5d 6d 7d 8d 9d Td Jd Qd Kd Ad 2h 3h 4h 5h 6h 7h 8h 9h Th Jh Qh Kh Ah 2s 3s 4s 5s 6s 7s 8s 9s Ts Js Qs Ks As");
    }

    #[test]
    fn from_string_to_hand() {
        let hand = Hand::from("2c Ts 9h 9s Ad".to_string());
        assert_eq!(hand.cards.len(), 5);
        assert_eq!(hand.to_string(), "2c Ts 9h 9s Ad");
        assert_eq!(hand.cards[1], Card::new(Suit::Spade, Rank::Ten));
    }
}
