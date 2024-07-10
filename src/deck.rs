// Shuffle a deck of card (from card.rs) randomly and return it.
// Include a struct for Deck and tests

use crate::card::{Card, Rank, Suit};
use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    // Returns a fresh deck
    pub fn new() -> Deck {
        let mut cards: Vec<Card> = Vec::new();
        for suit in Suit::iter() {
            for rank in Rank::iter() {
                cards.push(Card::new(*suit, *rank));
            }
        }
        Deck { cards }
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_new_deck() {
        let deck = Deck::new();
        assert_eq!(deck.cards.len(), 52);
    }
}
