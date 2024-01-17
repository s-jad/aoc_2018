use itertools::Itertools;
use std::{collections::HashMap, time::Instant};

fn find_str<'a, 'b, 'c>(
    mut possible_starts: Vec<char>,
    mut rest: HashMap<char, Vec<char>>,
) -> (Vec<char>, HashMap<char, Vec<char>>) {
    if rest.len() == 0 {
        return (possible_starts, rest);
    }

    let next = rest
        .iter()
        .filter_map(|(e, s_vec)| {
            if s_vec.len() == 0 || s_vec.iter().all(|s| possible_starts.contains(&s)) {
                Some(e)
            } else {
                None
            }
        })
        .min_by(|e1, e2| e1.cmp(e2));

    if next.is_some() {
        let n = *next.unwrap();
        rest.remove_entry(&n);

        if !possible_starts.contains(&n) {
            possible_starts.push(n);
        }
    }

    find_str(possible_starts, rest)
}

fn process(input: &str) -> String {
    let mut steps: HashMap<char, Vec<char>> = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter(|s| s.len() == 1)
                .map(|s| s.chars().next().unwrap())
                .collect_tuple::<(_, _)>()
                .unwrap()
        })
        .fold(HashMap::new(), |mut st, (s, e)| {
            let entry = st.entry(e).or_insert_with(Vec::new);
            entry.push(s);
            st
        });

    for i in b'A'..b'Z' {
        if !steps.keys().any(|k| *k as u8 == i) {
            steps.insert(i.into(), Vec::new());
        }
    }

    let mut possible_starts: Vec<char> = Vec::new();

    find_str(possible_starts, steps)
        .0
        .iter()
        .collect::<String>()
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
