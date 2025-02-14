// Shuffle a deck of card (from card.rs) randomly and return it.
// Include a struct for Deck and tests

use crate::card::{Card, Rank, Suit};
use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
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

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::thread_rng());
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
        assert_eq!(deck.cards[0], Card::new(Suit::Clubs, Rank::Deuce));
        assert_eq!(deck.cards[12], Card::new(Suit::Clubs, Rank::Ace));
        assert_eq!(deck.cards[13], Card::new(Suit::Diamonds, Rank::Deuce));
        assert_eq!(deck.cards[50], Card::new(Suit::Spades, Rank::King));
        assert_eq!(deck.cards[51], Card::new(Suit::Spades, Rank::Ace));
    }

    #[test]
    fn deck_should_shuffle() {
        let mut deck = Deck::new();
        assert_eq!(deck.cards.len(), 52);
        assert_eq!(deck.cards[0], Card::new(Suit::Clubs, Rank::Deuce));
        assert_eq!(deck.cards[25], Card::new(Suit::Diamonds, Rank::Ace));
        assert_eq!(deck.cards[50], Card::new(Suit::Spades, Rank::King));

        deck.shuffle();
        assert_eq!(deck.cards.len(), 52);
        assert!(
            (deck.cards[0] != Card::new(Suit::Clubs, Rank::Deuce))
                || (deck.cards[25] != Card::new(Suit::Diamonds, Rank::Ace))
                || (deck.cards[50] != Card::new(Suit::Spades, Rank::King))
        );
    }
}
