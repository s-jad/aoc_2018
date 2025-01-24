use itertools::Itertools;
use std::{collections::HashMap, time::Instant};

fn process(input: &str) -> usize {
    let (p1, p2) = input
        .split("\n\n")
        .collect_tuple::<(_, _)>()
        .expect("2 parts");

    let mut temp = p1
        .chars()
        .filter(|c| *c == '.' || *c == '#')
        .enumerate()
        .map(|(u, c)| (c, u as i32))
        .collect_vec();

    let l = temp.len() as i32;

    temp.push(('.', l));
    temp.push(('.', l + 1));
    temp.push(('.', l + 2));

    let mut state = [('.', -3), ('.', -2), ('.', -1)]
        .into_iter()
        .chain(temp)
        .collect_vec();

    let ins = p2.lines().fold(HashMap::new(), |mut map, l| {
        if let Some(c) = l.chars().nth_back(0) {
            if c == '#' {
                map.insert(l[0..=4].to_owned(), c);
            }
        }
        map
    });

    let mut sum: usize = 0;
    let mut diff: i32 = 0;
    let mut prev_diff = -1;
    let mut same_diff_count = 0;
    let mut idx: usize = 0;

    while same_diff_count < 2 {
        let prev_sum = sum;
        prev_diff = diff;
        let mut new_state = state.clone();

        for j in 0..state.len() - 4 {
            let win = state[j..j + 5].iter().map(|(c, i)| c).collect::<String>();

            if let Some(c) = ins.get(&win) {
                new_state[j + 2].0 = *c;
            } else {
                new_state[j + 2].0 = '.';
            }
        }

        match (
            new_state[3].0 == '#',
            new_state[new_state.len() - 3].0 == '#',
        ) {
            (true, true) => {
                let min = new_state[0].1;
                let max = new_state[new_state.len() - 1].1;

                state = [('.', min - 1)].into_iter().chain(new_state).collect_vec();
                state.push(('.', max + 1));
            }
            (true, false) => {
                let min = new_state[0].1;
                state = [('.', min - 1)].into_iter().chain(new_state).collect_vec();
            }
            (false, true) => {
                let max = new_state[new_state.len() - 1].1;
                state = new_state;
                state.push(('.', max + 1));
            }
            (false, false) => {
                state = new_state;
            }
        }

        sum = state
            .iter()
            .filter_map(|(c, i)| if *c == '#' { Some(i) } else { None })
            .sum::<i32>() as usize;

        diff = sum as i32 - prev_sum as i32;
        idx += 1;

        if diff == prev_diff {
            same_diff_count += 1;
        } else {
            same_diff_count = 0;
        }
    }

    ((50_000_000_000 - idx) * diff as usize) + sum
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
