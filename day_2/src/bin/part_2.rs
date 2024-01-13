use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> String {
    let ids = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    let (v1, v2) = ids
        .iter()
        .cartesian_product(ids.iter())
        .filter(|(v1, v2)| v1 != v2)
        .find_map(|(v1, v2)| {
            let diff_count = v1.iter().zip(v2.iter()).filter(|(c1, c2)| c1 != c2).count();

            if diff_count == 1 {
                Some((v1, v2))
            } else {
                None
            }
        })
        .unwrap();

    v1.iter()
        .zip(v2.iter())
        .fold(String::new(), |mut s, (c1, c2)| {
            if c1 == c2 {
                s.push(*c1);
            }
            s
        })
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
