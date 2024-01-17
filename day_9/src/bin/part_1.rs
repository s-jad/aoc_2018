use itertools::Itertools;
use std::{collections::HashMap, time::Instant};

fn process(input: &str) -> usize {
    let (players, winning) = input
        .split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect_tuple()
        .unwrap();

    let mut marbles = vec![0, 2, 1];
    let mut player_scores = HashMap::new();

    for player_id in 1..=players {
        player_scores.insert(player_id, 0);
    }

    let mut current_idx = 1usize;

    for m in 3..=winning {
        let m_len = marbles.len();

        if m % 23 == 0 {
            current_idx = (current_idx + m_len - 7) % m_len;
            let player = (m - 1) % players + 1;
            let score = player_scores.entry(player).or_insert(0);
            *score += marbles.remove(current_idx) + m;
        } else {
            let new_idx = current_idx + 2;
            if new_idx == m_len {
                marbles.push(m);
                current_idx = new_idx;
            } else {
                current_idx = new_idx % m_len;
                marbles.insert(current_idx, m);
            }
        }
    }

    *player_scores.values().max().unwrap()
}

fn main() {
    let input = include_str!("../../input.txt");

    let start = Instant::now();
    let output = process(input);
    let time = start.elapsed();

    dbg!(output, time);
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = include_str!("../../test.txt");
        let output = process(input);
        assert_eq!(result,);
    }
}
