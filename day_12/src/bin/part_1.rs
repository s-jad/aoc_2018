use itertools::Itertools;
use std::{collections::HashMap, time::Instant};

fn process(input: &str) -> i32 {
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

    for i in 0..20 {
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
    }

    state
        .iter()
        .filter_map(|(c, i)| if *c == '#' { Some(i) } else { None })
        .sum()
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
