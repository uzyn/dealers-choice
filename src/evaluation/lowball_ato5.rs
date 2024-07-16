use super::*;

pub struct LowballAto5 {}

impl LowballAto5 {
    // Ace is low
    const RANKS: [u32; 13] = [2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 1];
}

impl Evaluation for LowballAto5 {
    fn eval_hand(hand: &Hand) -> Result<u32, Error> {
        if hand.cards.len() != 5 {
            return Err(Error::InvalidHand);
        }
        let mut score: u32 = 0;
        for card in &hand.cards {
            score += Self::RANKS[card.rank as usize];
        }
        Ok(score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hand::Hand;

    #[test]
    fn test_eval_hand_valid() {
        assert_eq!(
            LowballAto5::eval_hand(&Hand::from("Ac 2c 3d 4h 5s".to_string())),
            Ok(31)
        );
        assert_eq!(
            LowballAto5::eval_hand(&Hand::from("Ac 2d 3h 4s 6c".to_string())),
            Ok(47)
        );
        assert_eq!(
            LowballAto5::eval_hand(&Hand::from("2c 3d 4h 5s 6c".to_string())),
            Ok(62)
        );
    }

    #[test]
    fn test_eval_hand_invalid() {
        assert_eq!(LowballAto5::eval_hand(&Hand::from("Ac 2c 3d 4h".to_string())), Err(Error::InvalidHand));
        assert!(LowballAto5::eval_hand(&Hand::from("Ac 2c 3d 4h 5c".to_string())).is_ok());
        assert_eq!(LowballAto5::eval_hand(&Hand::from("Ac 2c 3d 4h 5c 6d".to_string())), Err(Error::InvalidHand));
    }
}
