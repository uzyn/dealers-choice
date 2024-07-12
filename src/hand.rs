use crate::card::Card;

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Hand {
        Hand { cards: Vec::new() }
    }

    //TODO: Sort and display of Hand
}

impl Default for Hand {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_new_hand() {
        let hand = Hand::new();
        assert_eq!(hand.cards.len(), 0);
    }
}
