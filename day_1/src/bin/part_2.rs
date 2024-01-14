use itertools::Itertools;
use std::{collections::HashSet, time::Instant};

fn process(input: &str) -> i32 {
    let mut seen = HashSet::new();
    let mut acc = 0;
    let mut ans = 0;

    let nums = input
        .lines()
        .map(|s| s.trim_start_matches('+').parse::<i32>().unwrap())
        .cycle();

    for num in nums {
        acc += num;
        if !seen.insert(acc) {
            ans = acc;
            break;
        }
    }

    ans
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
