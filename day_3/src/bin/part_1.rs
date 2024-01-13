use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    let claims = input
        .lines()
        .fold(vec![], |mut c: Vec<((usize, usize), (usize, usize))>, s| {
            let parts = s
                .split_terminator(&['@', ':', ',', 'x'])
                .filter_map(|s| s.trim().parse::<usize>().ok())
                .collect_tuple::<(_, _, _, _)>()
                .unwrap();

            c.push(((parts.0, parts.1), (parts.0 + parts.2, parts.1 + parts.3)));

            c
        });

    let max_x = claims.iter().map(|c| c.1 .0).max().unwrap();
    let max_y = claims.iter().map(|c| c.1 .1).max().unwrap();

    let mut grid = vec![vec![0; max_y + 1]; max_x + 1];

    for (c_start, c_end) in &claims {
        for x in c_start.0..c_end.0 {
            for y in c_start.1..c_end.1 {
                grid[x][y] += 1;
            }
        }
    }

    grid.iter()
        .flat_map(|row| row.iter().filter(|&&c| c >= 2))
        .count()
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
