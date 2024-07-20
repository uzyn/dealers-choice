use super::*;

const MAX_SCORE: u64 = u64::MAX;

pub struct LowballAto5 {}

impl LowballAto5 {
    // Ace is low
    const RANKS: [u64; 13] = [2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 1];
    const PAIR_MULTIPLIER: u64 = 1 << 13;
    const TRIP_MULTIPLIER: u64 = 1 << (13 * 2);
    const QUAD_MULTIPLIER: u64 = 1  << (13 * 3);
}

impl Evaluation for LowballAto5 {
    fn eval_hand(hand: &Hand) -> Result<u64, Error> {
        if hand.cards.len() != 5 {
            return Err(Error::InvalidHand);
        }

        let mut frequencies: [u8; 13] = [0; 13];
        let mut score: u64 = 0;
        for card in &hand.cards {
            // score += Self::RANKS[card.rank as usize];
            frequencies[card.rank as usize] += 1;
        }

        for (index, &freq) in frequencies.iter().enumerate() {
            match freq {
                0 => (),
                1 => score += Self::RANKS[index],
                2 => score += Self::PAIR_MULTIPLIER * Self::RANKS[index],
                3 => score += Self::TRIP_MULTIPLIER * Self::RANKS[index],
                4 => score += Self::QUAD_MULTIPLIER * Self::RANKS[index],
                _ => return Err(Error::InvalidHand),
            }
        }
        dbg!(hand.to_string(), score);

        // Reverse the score for lowball, smaller hand should return higher score
        let low_score: u64 = MAX_SCORE - score;
        Ok(low_score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hand::Hand;
    use std::cmp::Ordering;

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
        assert_eq!(
            LowballAto5::eval_hand(&Hand::try_from("Ac Ad Ac Ah As".to_string()).unwrap()),
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
        let h_aajqk = Hand::try_from("Ac Ac Kc Jd Qh".to_string()).unwrap();
        let h_kk234 = Hand::try_from("Kc 2c Kh 3d 4h".to_string()).unwrap();
        let h_55jqk = Hand::try_from("5c Jc Qc Ks 5h".to_string()).unwrap();
        let h_22345 = Hand::try_from("2c 2d 3h 4c 5h".to_string()).unwrap();

        assert_eq!(
            LowballAto5::compare_hands(&h_aajqk, &h_best),
            Ordering::Less
        );
        // Pair always loses to high-card
        assert_eq!(
            LowballAto5::compare_hands(&h_aajqk, &h_9tjqk),
            Ordering::Less
        );
        // Aces are low
        assert_eq!(
            LowballAto5::compare_hands(&h_aajqk, &h_55jqk),
            Ordering::Greater
        );
        assert_eq!(
            LowballAto5::compare_hands(&h_aajqk, &h_kk234),
            Ordering::Greater
        );
        assert_eq!(
            LowballAto5::compare_hands(&h_aajqk, &h_22345),
            Ordering::Greater
        );

        // Two pairs
        let h_aa223 = Hand::try_from("Ac Ac 2c 2d 3h".to_string()).unwrap();
        let h_kk223 = Hand::try_from("Kc Kc 2c 2d 3h".to_string()).unwrap();
        let h_55jjq = Hand::try_from("5c Jc Qc 5h Jh".to_string()).unwrap();

        assert_eq!(
            LowballAto5::compare_hands(&h_aa223, &h_aajqk),
            Ordering::Less
        );
        assert_eq!(
            LowballAto5::compare_hands(&h_aa223, &h_kk223),
            Ordering::Greater
        );
        assert_eq!(
            LowballAto5::compare_hands(&h_aa223, &h_55jjq),
            Ordering::Greater
        );
        // Two pairs always loses to one pair
        assert_eq!(
            LowballAto5::compare_hands(&h_aa223, &h_kk234),
            Ordering::Less
        );

        // Trips
        let h_aaa23 = Hand::try_from("Ac Ac Ac 2c 3h".to_string()).unwrap();
        let h_kkk23 = Hand::try_from("Kc Kc Kc 2c 3h".to_string()).unwrap();
        let h_555jq = Hand::try_from("5c Jc Qc 5h 5s".to_string()).unwrap();

        assert_eq!(
            LowballAto5::compare_hands(&h_aaa23, &h_aa223),
            Ordering::Less
        );
        assert_eq!(
            LowballAto5::compare_hands(&h_aaa23, &h_kkk23),
            Ordering::Greater
        );
        assert_eq!(
            LowballAto5::compare_hands(&h_aaa23, &h_555jq),
            Ordering::Greater
        );
        // Trips always loses to two pairs
        assert_eq!(
            LowballAto5::compare_hands(&h_aaa23, &h_kk223),
            Ordering::Less
        );

        // Quads
        let h_aaaak = Hand::try_from("Ac Ac Ac Ac Kh".to_string()).unwrap();
        let h_kkkk2 = Hand::try_from("Kc Kc Kc Kc 2h".to_string()).unwrap();

        assert_eq!(
            LowballAto5::compare_hands(&h_aaaak, &h_aaa23),
            Ordering::Less
        );
        assert_eq!(
            LowballAto5::compare_hands(&h_aaaak, &h_kkkk2),
            Ordering::Greater
        );
        // Quads always loses to trips
        assert_eq!(
            LowballAto5::compare_hands(&h_aaaak, &h_kkk23),
            Ordering::Less
        );
    }
}
