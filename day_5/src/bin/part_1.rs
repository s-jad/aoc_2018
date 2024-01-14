use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    let mut s = input.trim().chars().collect_vec();

    let mut changes = 1;

    while changes != 0 {
        changes = 0;

        for i in 0..s.len() - 1 {
            if i < s.len() - 1 {
                let c1 = s[i];
                let c2 = s[i + 1];

                if c1.to_uppercase().next() == c2.to_uppercase().next() && c1 != c2 {
                    changes += 1;
                    s.drain(i..=(i + 1));
                }
            } else {
                break;
            }
        }
    }
    s.len()
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
