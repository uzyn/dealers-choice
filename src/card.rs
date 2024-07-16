#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

// Return "c" for Club, "d" for Diamond, "h" for Heart, and "s" for Spade
impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Suit::Club => write!(f, "c"),
            Suit::Diamond => write!(f, "d"),
            Suit::Heart => write!(f, "h"),
            Suit::Spade => write!(f, "s"),
        }
    }
}

impl Suit {
    pub fn iter() -> &'static [Suit] {
        static SUITS: [Suit; 4] = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];
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
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

impl From<String> for Card {
    fn from(s: String) -> Card {
        let suit = match s.chars().last().unwrap() {
            'c' => Suit::Club,
            'd' => Suit::Diamond,
            'h' => Suit::Heart,
            's' => Suit::Spade,
            _ => panic!("Invalid suit"),
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
            _ => panic!("Invalid rank"),
        };
        Card { suit, rank }
    }
}

// impl Ord for Card {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.suit.cmp(&other.suit).then_with(|| self.rank.cmp(&other.rank))
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_card_in_string_format() {
        assert_eq!(
            Card {
                suit: Suit::Club,
                rank: Rank::Ace
            }
            .to_string(),
            "Ac"
        );
        assert_eq!(
            Card {
                suit: Suit::Heart,
                rank: Rank::Ten
            }
            .to_string(),
            "Th"
        );
        assert_eq!(
            Card {
                suit: Suit::Spade,
                rank: Rank::Queen
            }
            .to_string(),
            "Qs"
        );
        assert_eq!(
            Card {
                suit: Suit::Diamond,
                rank: Rank::Trey
            }
            .to_string(),
            "3d"
        );
    }

    #[test]
    fn card_from_string() {
        assert_eq!(
            Card::from("Ac".to_string()),
            Card {
                suit: Suit::Club,
                rank: Rank::Ace
            }
        );
        assert_eq!(
            Card::from("Th".to_string()),
            Card {
                suit: Suit::Heart,
                rank: Rank::Ten
            }
        );
        assert_eq!(
            Card::from("Qs".to_string()),
            Card {
                suit: Suit::Spade,
                rank: Rank::Queen
            }
        );

        let card = Card {
            suit: Suit::Diamond,
            rank: Rank::Trey,
        };
        assert_eq!(Card::from(card.to_string()), card);
    }

    // Test card score, use Card::from<string> and not Card::new
    #[test]
    fn test_card_ord_position() {
        let card1 = Card::from("As".to_string());
        let card2 = Card::from("2c".to_string());
        let card3 = Card::from("Ad".to_string());
        let card4 = Card::from("2d".to_string());

        assert_eq!(card1.ord_position(OrderFirstBy::Suit), 3 * 13 + 12);
        assert_eq!(card2.ord_position(OrderFirstBy::Suit), 0 * 13 + 0);
        assert_eq!(card3.ord_position(OrderFirstBy::Suit), 1 * 13 + 12);
        assert_eq!(card4.ord_position(OrderFirstBy::Suit), 1 * 13 + 0);

        assert_eq!(card1.ord_position(OrderFirstBy::Rank), 12 * 4 + 3);
        assert_eq!(card2.ord_position(OrderFirstBy::Rank), 0 * 4 + 0);
        assert_eq!(card3.ord_position(OrderFirstBy::Rank), 12 * 4 + 1);
        assert_eq!(card4.ord_position(OrderFirstBy::Rank), 0 * 4 + 1);
    }
}
