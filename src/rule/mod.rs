use crate::hand::Hand;


pub trait Rule {
    fn evaluate_hand(hand: &Hand, board: Option<&Hand>) -> u32;

    fn compare_hands(hand1: &Hand, hand2: &Hand, board: Option<&Hand>) -> std::cmp::Ordering {
        let score1 = Self::evaluate_hand(hand1, board);
        let score2 = Self::evaluate_hand(hand2, board);
        score1.cmp(&score2)
    }

    // Returns a vector of the same size as players with the percentage of pot won
    // If it's a tie, and assuming 2 players, returned value would be vec!<0.5, 0.5>
    fn determine_winner(players: &Vec<Hand>, board: Option<&Hand>) -> Vec<f32> {
        // Default logic for single-pot games, e.g. not hi-lo.
        let mut winner = 0;
        for i in 1..players.len() {
            if Self::compare_hands(&players[i], &players[winner], board) == std::cmp::Ordering::Greater {
                winner = i;
            }
        }

        // Check for ties
        let mut winners = vec![false; players.len()];
        let mut winner_count = 0;
        for i in 1..players.len() {
            if Self::compare_hands(&players[i], &players[winner], board) == std::cmp::Ordering::Equal {
                winner_count += 1;
                winners[i] = true;
            }
        }

        let mut payouts: Vec<f32> = vec![0.0; players.len()];
        let pot_split: f32 = 1.0 / winner_count as f32;
        for i in 1..players.len() {
            if winners[i] {
                payouts[i] = pot_split;
            }
        }

        payouts
    }
}
