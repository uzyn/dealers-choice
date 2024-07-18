use crate::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

// Return "c" for Club, "d" for Diamond, "h" for Heart, and "s" for Spade
impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Suit::Clubs => write!(f, "c"),
            Suit::Diamonds => write!(f, "d"),
            Suit::Hearts => write!(f, "h"),
            Suit::Spades => write!(f, "s"),
        }
    }
}

impl Suit {
    pub fn iter() -> &'static [Suit] {
        static SUITS: [Suit; 4] = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
        &SUITS
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    Deuce,
    Trey,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rank::Deuce => write!(f, "2"),
            Rank::Trey => write!(f, "3"),
            Rank::Four => write!(f, "4"),
            Rank::Five => write!(f, "5"),
            Rank::Six => write!(f, "6"),
            Rank::Seven => write!(f, "7"),
            Rank::Eight => write!(f, "8"),
            Rank::Nine => write!(f, "9"),
            Rank::Ten => write!(f, "T"),
            Rank::Jack => write!(f, "J"),
            Rank::Queen => write!(f, "Q"),
            Rank::King => write!(f, "K"),
            Rank::Ace => write!(f, "A"),
        }
    }
}

impl Rank {
    pub fn iter() -> &'static [Rank] {
        static RANKS: [Rank; 13] = [
            Rank::Deuce,
            Rank::Trey,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace,
        ];
        &RANKS
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OrderFirstBy {
    Suit,
    Rank,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Card {
        Card { suit, rank }
    }

    pub fn ord_position(&self, order_first_by: OrderFirstBy) -> u8 {
        match order_first_by {
            OrderFirstBy::Suit => (self.suit as u8) * 13 + (self.rank as u8),
            OrderFirstBy::Rank => (self.rank as u8) * 4 + (self.suit as u8),
        }
    }

    fn cmp_ord_first_by(&self, other: &Card, order_first_by: OrderFirstBy) -> std::cmp::Ordering {
        match order_first_by {
            OrderFirstBy::Rank => self
                .ord_position(OrderFirstBy::Rank)
                .cmp(&other.ord_position(OrderFirstBy::Rank)),
            OrderFirstBy::Suit => self
                .ord_position(OrderFirstBy::Suit)
                .cmp(&other.ord_position(OrderFirstBy::Suit)),
        }
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

impl TryFrom<String> for Card {
    type Error = Error;

    fn try_from(s: String) -> Result<Card, Error> {
        if s.len() != 2 {
            return Err(Error::InvalidCardNotation);
        }

        let suit = match s.chars().last().unwrap() {
            'c' => Suit::Clubs,
            'd' => Suit::Diamonds,
            'h' => Suit::Hearts,
            's' => Suit::Spades,
            _ => return Err(Error::InvalidCardNotation),
        };
        let rank = match s.chars().nth(0).unwrap() {
            '2' => Rank::Deuce,
            '3' => Rank::Trey,
            '4' => Rank::Four,
            '5' => Rank::Five,
            '6' => Rank::Six,
            '7' => Rank::Seven,
            '8' => Rank::Eight,
            '9' => Rank::Nine,
            'T' => Rank::Ten,
            'J' => Rank::Jack,
            'Q' => Rank::Queen,
            'K' => Rank::King,
            'A' => Rank::Ace,
            _ => return Err(Error::InvalidCardNotation),
        };
        Ok(Card { suit, rank })
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Card) -> std::cmp::Ordering {
        self.cmp_ord_first_by(other, OrderFirstBy::Rank) // default via Rank
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_card_in_string_format() {
        assert_eq!(
            Card {
                suit: Suit::Clubs,
                rank: Rank::Ace
            }
            .to_string(),
            "Ac"
        );
        assert_eq!(
            Card {
                suit: Suit::Hearts,
                rank: Rank::Ten
            }
            .to_string(),
            "Th"
        );
        assert_eq!(
            Card {
                suit: Suit::Spades,
                rank: Rank::Queen
            }
            .to_string(),
            "Qs"
        );
        assert_eq!(
            Card {
                suit: Suit::Diamonds,
                rank: Rank::Trey
            }
            .to_string(),
            "3d"
        );
    }

    #[test]
    fn card_from_string() {
        assert_eq!(
            Card::try_from("Ac".to_string()).unwrap(),
            Card {
                suit: Suit::Clubs,
                rank: Rank::Ace
            }
        );
        assert_eq!(
            Card::try_from("Th".to_string()).unwrap(),
            Card {
                suit: Suit::Hearts,
                rank: Rank::Ten
            }
        );
        assert_eq!(
            Card::try_from("Qs".to_string()).unwrap(),
            Card {
                suit: Suit::Spades,
                rank: Rank::Queen
            }
        );

        let card = Card {
            suit: Suit::Diamonds,
            rank: Rank::Trey,
        };
        assert_eq!(Card::try_from(card.to_string()).unwrap(), card);
    }

    #[test]
    fn test_card_ord_position() {
        let card1 = Card::try_from("As".to_string()).unwrap();
        let card2 = Card::try_from("2c".to_string()).unwrap();
        let card3 = Card::try_from("Ad".to_string()).unwrap();
        let card4 = Card::try_from("2d".to_string()).unwrap();

        assert_eq!(card1.ord_position(OrderFirstBy::Suit), 0 * 13 + 12);
        assert_eq!(card2.ord_position(OrderFirstBy::Suit), 3 * 13 + 0);
        assert_eq!(card3.ord_position(OrderFirstBy::Suit), 2 * 13 + 12);
        assert_eq!(card4.ord_position(OrderFirstBy::Suit), 2 * 13 + 0);

        assert_eq!(card1.ord_position(OrderFirstBy::Rank), 12 * 4 + 0);
        assert_eq!(card2.ord_position(OrderFirstBy::Rank), 0 * 4 + 3);
        assert_eq!(card3.ord_position(OrderFirstBy::Rank), 12 * 4 + 2);
        assert_eq!(card4.ord_position(OrderFirstBy::Rank), 0 * 4 + 2);
    }

    #[test]
    fn test_card_try_from_error() {
        assert_eq!(
            Card::try_from("A".to_string()),
            Err(Error::InvalidCardNotation)
        );
        assert_eq!(
            Card::try_from("Acx".to_string()),
            Err(Error::InvalidCardNotation)
        );
        assert_eq!(
            Card::try_from("1c".to_string()),
            Err(Error::InvalidCardNotation)
        );
        assert_eq!(
            Card::try_from("Bc".to_string()),
            Err(Error::InvalidCardNotation)
        );
        assert_eq!(
            Card::try_from("AD".to_string()),
            Err(Error::InvalidCardNotation)
        );
        assert_eq!(
            Card::try_from("Ad ".to_string()), // untrimmed
            Err(Error::InvalidCardNotation)
        );
        assert!(Card::try_from("Ad".to_string()).is_ok());
    }

    #[test]
    fn test_card_cmp() {
        let card1 = Card::try_from("As".to_string()).unwrap();
        let card2 = Card::try_from("2c".to_string()).unwrap();
        let card3 = Card::try_from("Ad".to_string()).unwrap();
        let card4 = Card::try_from("2d".to_string()).unwrap();

        assert_eq!(
            card1.cmp_ord_first_by(&card2, OrderFirstBy::Rank),
            std::cmp::Ordering::Greater
        );
        assert_eq!(
            card2.cmp_ord_first_by(&card1, OrderFirstBy::Rank),
            std::cmp::Ordering::Less
        );
        assert_eq!(
            card1.cmp_ord_first_by(&card3, OrderFirstBy::Rank),
            std::cmp::Ordering::Less
        );
        assert_eq!(
            card2.cmp_ord_first_by(&card4, OrderFirstBy::Rank),
            std::cmp::Ordering::Greater
        );

        assert_eq!(
            card1.cmp_ord_first_by(&card2, OrderFirstBy::Suit),
            std::cmp::Ordering::Less
        );
        assert_eq!(
            card2.cmp_ord_first_by(&card1, OrderFirstBy::Suit),
            std::cmp::Ordering::Greater
        );
        assert_eq!(
            card1.cmp_ord_first_by(&card3, OrderFirstBy::Suit),
            std::cmp::Ordering::Less
        );
        assert_eq!(
            card2.cmp_ord_first_by(&card4, OrderFirstBy::Suit),
            std::cmp::Ordering::Greater
        );

        assert_eq!(card1.cmp(&card2), std::cmp::Ordering::Greater);
        assert_eq!(card2.cmp(&card1), std::cmp::Ordering::Less);
        assert_eq!(card1.cmp(&card3), std::cmp::Ordering::Less);
        assert_eq!(card2.cmp(&card4), std::cmp::Ordering::Greater);
    }

    #[test]
    fn test_card_ord() {
        let card1 = Card::try_from("As".to_string()).unwrap();
        let card2 = Card::try_from("2c".to_string()).unwrap();
        let card3 = Card::try_from("Ad".to_string()).unwrap();
        let card4 = Card::try_from("2d".to_string()).unwrap();

        assert!(card1 > card2);
        assert!(card2 < card1);
        assert!(card1 < card3);
        assert!(card2 > card4);
    }
}
