use super::*;

const MAX_SCORE: u128 = u128::MAX;

#[derive(Default)]
pub struct LowballAto5 {}

impl LowballAto5 {
    // Ace is low
    const RANKS: [u128; 13] = [2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 1];
    const ONE_PAIR_MULTIPLIER: u128 = 1 << 13;
    const TWO_PAIRS_MULTIPLIER: u128 = 1 << (13 * 2);
    const TRIPS_MULTIPLIER: u128 = 1 << (13 * 3);
    const QUADS_MULTIPLIER: u128 = 1 << (13 * 4);
}

impl EvalHand for LowballAto5 {
    fn eval_hand(hand: &Hand) -> Result<u128, Error> {
        if hand.cards.len() != 5 {
            return Err(Error::InvalidHand);
        }

        let mut frequencies: [u8; 13] = [0; 13];
        let mut score: u128 = 0;
        let mut two_pairs_count: u8 = 0;
        for card in &hand.cards {
            frequencies[card.rank as usize] += 1;
            match frequencies[card.rank as usize] {
                2 => two_pairs_count += 1,
                3 => two_pairs_count -= 1, // improved to trips
                _ => (),
            }
        }

        for (index, &freq) in frequencies.iter().enumerate() {
            match freq {
                0 => (),
                1 => score += Self::RANKS[index],
                2 => {
                    score += Self::RANKS[index]
                        * match two_pairs_count {
                            1 => Self::ONE_PAIR_MULTIPLIER,
                            2 => Self::TWO_PAIRS_MULTIPLIER,
                            _ => return Err(Error::InvalidHand),
                        }
                }
                3 => score += Self::TRIPS_MULTIPLIER * Self::RANKS[index],
                4 => score += Self::QUADS_MULTIPLIER * Self::RANKS[index],
                _ => return Err(Error::InvalidHand),
            }
        }

        // Reverse the score for lowball, smaller hand should return higher score
        let low_score: u128 = MAX_SCORE - score;
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
            LowballAto5::eval_hand(&Hand::try_from("Ac 2c 3d 4h 5s").unwrap()),
            Ok(MAX_SCORE - 31)
        );
        assert_eq!(
            LowballAto5::eval_hand(&Hand::try_from("Ac 2d 3h 4s 6c").unwrap()),
            Ok(MAX_SCORE - 47)
        );
        assert_eq!(
            LowballAto5::eval_hand(&Hand::try_from("2c 3d 4h 5s 6c").unwrap()),
            Ok(MAX_SCORE - 62)
        );
    }

    #[test]
    fn test_eval_hand_invalid() {
        assert_eq!(
            LowballAto5::eval_hand(&Hand::try_from("Ac 2c 3d 4h").unwrap()),
            Err(Error::InvalidHand)
        );
        assert!(LowballAto5::eval_hand(&Hand::try_from("Ac 2c 3d 4h 5c").unwrap()).is_ok());
        assert_eq!(
            LowballAto5::eval_hand(&Hand::try_from("Ac 2c 3d 4h 5c 6d").unwrap()),
            Err(Error::InvalidHand)
        );
        assert_eq!(
            LowballAto5::eval_hand(&Hand::try_from("Ac Ad Ac Ah As").unwrap()),
            Err(Error::InvalidHand)
        );
    }

    #[test]
    fn test_compare_hands() {
        // High-card hands
        let h_best = Hand::try_from("Ac 2c 3d 4h 5s").unwrap();
        let h_a2345 = Hand::try_from("Ad 2s 3c 4c 5h").unwrap();
        let h_a2346 = Hand::try_from("Ad 2h 3s 4c 6c").unwrap();
        let h_a234t = Hand::try_from("Ac 2c 3d 4h Ts").unwrap();
        let h_56789 = Hand::try_from("5c 6c 7d 8h 9s").unwrap();
        let h_9tjqk = Hand::try_from("9c Td Jc Qc Ks").unwrap();

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
        let h_aajqk = Hand::try_from("Ac Ac Kc Jd Qh").unwrap();
        let h_kk234 = Hand::try_from("Kc 2c Kh 3d 4h").unwrap();
        let h_55jqk = Hand::try_from("5c Jc Qc Ks 5h").unwrap();
        let h_22345 = Hand::try_from("2c 2d 3h 4c 5h").unwrap();

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
        let h_aa223 = Hand::try_from("Ac Ac 2c 2d 3h").unwrap();
        let h_kk223 = Hand::try_from("Kc Kc 2c 2d 3h").unwrap();
        let h_55jjq = Hand::try_from("5c Jc Qc 5h Jh").unwrap();

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
        let h_aaa23 = Hand::try_from("Ac Ac Ac 2c 3h").unwrap();
        let h_kkkjq = Hand::try_from("Kc Kc Kc Jc Qh").unwrap();
        let h_555jq = Hand::try_from("5c Jc Qc 5h 5s").unwrap();

        assert_eq!(
            LowballAto5::compare_hands(&h_aaa23, &h_aa223),
            Ordering::Less
        );
        assert_eq!(
            LowballAto5::compare_hands(&h_aaa23, &h_kkkjq),
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

        // Fullhouse
        let h_aakkk = Hand::try_from("Ac Ac Kc Kd Kh").unwrap();
        let h_aaakk = Hand::try_from("Ac Ac Ac Kc Kh").unwrap();

        assert_eq!(
            LowballAto5::compare_hands(&h_aakkk, &h_aa223),
            Ordering::Less
        );
        assert_eq!(
            LowballAto5::compare_hands(&h_aakkk, &h_aaakk),
            Ordering::Less
        );
        // Fullhouse always loses to trips
        assert_eq!(
            LowballAto5::compare_hands(&h_aakkk, &h_kkkjq),
            Ordering::Less
        );

        // Quads
        let h_aaaak = Hand::try_from("Ac Ac Ac Ac Kh").unwrap(); // ensure score does not overflow
        let h_kkkk2 = Hand::try_from("Kc Kc Kc Kc 2h").unwrap();

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
            LowballAto5::compare_hands(&h_aaaak, &h_kkkjq),
            Ordering::Less
        );
        // Quads always loses to fullhouse
        assert_eq!(
            LowballAto5::compare_hands(&h_aaaak, &h_aakkk),
            Ordering::Less
        );
    }

    #[test]
    fn test_direct_comparison() {
        let h_aaaak = Evaluation::<LowballAto5>::try_from("Ac Ac Ac Ac Kh").unwrap();
        let h_kkkkq = Evaluation::<LowballAto5>::try_from("Kc Kc Kc Kc Qh").unwrap();
        let h_kkkqq = Evaluation::<LowballAto5>::try_from("Kc Kc Kc Qc Qh").unwrap();
        let h_aaa22 = Evaluation::<LowballAto5>::try_from("Ac Ac Ah 2c 2d").unwrap();
        let h_aa223 = Evaluation::<LowballAto5>::try_from("Ac Ac 2c 2h 3s").unwrap();
        let h_kkjqt = Evaluation::<LowballAto5>::try_from("Kc Kd Jc Qc Td").unwrap();
        let h_kkjqt_ds = Evaluation::<LowballAto5>::try_from("Ks Js Ts Qd Kh").unwrap();
        let h_a2345 = Evaluation::<LowballAto5>::try_from("Ad 2s 3c 4c 5h").unwrap();
        let h_a2346 = Evaluation::<LowballAto5>::try_from("Ad 2h 3s 4c 6c").unwrap();

        assert!(h_aaaak > h_kkkkq); // Ace is low
        assert!(h_kkkqq > h_aaaak); // Full house beats 4 of a kind
        assert!(h_aaa22 > h_kkkqq);
        assert!(h_aa223 > h_aaa22); // Two pairs beats full house
        assert!(h_kkjqt > h_aa223);
        assert!(h_kkjqt_ds == h_kkjqt); // Suits do not matter
        assert!(h_a2345 > h_a2346);
        assert!(h_a2346 > h_kkjqt);
    }
}
