use super::*;

pub struct LowballAto5 {}

impl LowballAto5 {
    // Ace is low
    const RANKS: [u32; 13] = [2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 1];
    const MULTIPLIER_COUNT: [u32; 4] = [0, 20_000, 50_000, 100_000]; // duplicate cards (pairs, trips, quads)
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

        // Reverse the score for lowball, smaller hand should return higher score
        const MAX_SCORE: u32 = 200_000; // > (4096 * 5) + 100000;
        let low_score: u32 = MAX_SCORE - score;

        Ok(low_score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hand::Hand;
    use std::cmp::Ordering;
    const MAX_SCORE: u32 = 200_000;

    #[test]
    fn test_eval_hand_valid() {
        assert_eq!(
            LowballAto5::eval_hand(&Hand::try_from("Ac 2c 3d 4h 5s".to_string()).unwrap()),
            Ok(MAX_SCORE - 31)
        );
        assert_eq!(
            LowballAto5::eval_hand(&Hand::try_from("Ac 2d 3h 4s 6c".to_string()).unwrap()),
            Ok(MAX_SCORE - 47)
        );
        assert_eq!(
            LowballAto5::eval_hand(&Hand::try_from("2c 3d 4h 5s 6c".to_string()).unwrap()),
            Ok(MAX_SCORE - 62)
        );
    }

    #[test]
    fn test_eval_hand_invalid() {
        assert_eq!(
            LowballAto5::eval_hand(&Hand::try_from("Ac 2c 3d 4h".to_string()).unwrap()),
            Err(Error::InvalidHand)
        );
        assert!(
            LowballAto5::eval_hand(&Hand::try_from("Ac 2c 3d 4h 5c".to_string()).unwrap()).is_ok()
        );
        assert_eq!(
            LowballAto5::eval_hand(&Hand::try_from("Ac 2c 3d 4h 5c 6d".to_string()).unwrap()),
            Err(Error::InvalidHand)
        );
    }

    #[test]
    fn test_compare_hands() {
        // High-card hands
        let h_best = Hand::try_from("Ac 2c 3d 4h 5s".to_string()).unwrap();
        let h_a2345 = Hand::try_from("Ad 2s 3c 4c 5h".to_string()).unwrap();
        let h_a2346 = Hand::try_from("Ad 2h 3s 4c 6c".to_string()).unwrap();
        let h_a234t = Hand::try_from("Ac 2c 3d 4h Ts".to_string()).unwrap();
        let h_56789 = Hand::try_from("5c 6c 7d 8h 9s".to_string()).unwrap();
        let h_9tjqk = Hand::try_from("9c Td Jc Qc Ks".to_string()).unwrap();

        assert_eq!(
            LowballAto5::compare_hands(&h_best, &h_a2346),
            Ordering::Greater
        );
        assert_eq!(
            LowballAto5::compare_hands(&h_best, &h_a2345),
            Ordering::Equal
        );
        assert_eq!(
            LowballAto5::compare_hands(&h_best, &h_a234t),
            Ordering::Greater
        );
        assert_eq!(
            LowballAto5::compare_hands(&h_a234t, &h_56789),
            Ordering::Less
        );
        assert_eq!(
            LowballAto5::compare_hands(&h_9tjqk, &h_56789),
            Ordering::Less
        );

        // Pair
        let h_aa234 = Hand::try_from("Ac Ac 2c 3d 4h".to_string()).unwrap();
        let h_kk234 = Hand::try_from("Kc Kc 2c 3d 4h".to_string()).unwrap();
        let h_55jqk = Hand::try_from("5c 5c Jc Qc Ks".to_string()).unwrap();

        assert_eq!(
            LowballAto5::compare_hands(&h_aa234, &h_best),
            Ordering::Less
        );
        assert_eq!(
            LowballAto5::compare_hands(&h_aa234, &h_9tjqk),
            Ordering::Less
        );
        assert_eq!(
            // Aces are low
            LowballAto5::compare_hands(&h_aa234, &h_55jqk),
            Ordering::Greater
        );
        assert_eq!(
            LowballAto5::compare_hands(&h_aa234, &h_kk234),
            Ordering::Greater
        );
    }
}
