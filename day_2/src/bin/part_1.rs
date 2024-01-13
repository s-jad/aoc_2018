use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    let ids = input.lines().map(|l| l.chars().sorted()).collect_vec();

    let mut num_2 = 0;
    let mut num_3 = 0;

    for id in ids {
        let counts = id.counts();

        if counts.values().contains(&2) {
            num_2 += 1;
        }

        if counts.values().contains(&3) {
            num_3 += 1;
        }
    }

    num_2 * num_3
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
