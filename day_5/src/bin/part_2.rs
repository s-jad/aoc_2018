use itertools::Itertools;
use std::time::Instant;

fn get_polymer_len(s: &[char], u: u8, l: u8) -> usize {
    let upper = u as char;
    let lower = l as char;
    let mut changes = 1;

    let mut new_s = s
        .iter()
        .filter(|&&c| c != upper && c != lower)
        .collect_vec();

    while changes != 0 {
        changes = 0;

        for i in 0..new_s.len() - 1 {
            if i < new_s.len() - 1 {
                let c1 = new_s[i];
                let c2 = new_s[i + 1];

                if c1.to_uppercase().next() == c2.to_uppercase().next() && c1 != c2 {
                    changes += 1;
                    new_s.drain(i..=(i + 1));
                }
            } else {
                break;
            }
        }
    }
    new_s.len()
}

fn process(input: &str) -> usize {
    let mut s = input.trim().chars().collect_vec();

    let mut min = 10000000;

    for u in b'A'..=b'Z' {
        let l = u + 32;

        let len = get_polymer_len(&s, u, l);

        if len < min {
            min = len;
        }
    }

    min
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
