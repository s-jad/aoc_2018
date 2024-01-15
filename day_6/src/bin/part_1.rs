use itertools::Itertools;
use std::{time::Instant, vec};

fn find_closest(coords: &[(usize, (usize, usize))], (x, y): (usize, usize)) -> usize {
    let mut min = std::usize::MAX;
    let mut min_id = std::usize::MAX;
    let mut count = 0;

    for (id, (cx, cy)) in coords.iter() {
        let dist = ((x as i32 - *cx as i32).abs() + (y as i32 - *cy as i32).abs()) as usize;

        if dist < min {
            min = dist;
            min_id = *id;
            count = 1;
        } else if dist == min {
            count += 1;
        }
    }

    if count == 1 {
        min_id
    } else {
        0usize
    }
}

fn process(input: &str) -> usize {
    let coords = input
        .lines()
        .enumerate()
        .map(|(id, l)| {
            (
                id + 1,
                l.split_terminator(&[',', ' '])
                    .filter_map(|s| s.parse::<usize>().ok())
                    .collect_tuple::<(_, _)>()
                    .unwrap(),
            )
        })
        .collect_vec();

    let max_x = coords.iter().map(|(_, (x, _))| x).max().unwrap() + 1;
    let max_y = coords.iter().map(|(_, (_, y))| y).max().unwrap() + 1;

    let mut grid = vec![vec![999999; max_x]; max_y];

    for y in 0..max_y {
        for x in 0..max_x {
            let id = find_closest(&coords, (x, y));
            grid[y][x] = id;
        }
    }

    let mut max_area = 0;

    for i in 1..=coords.len() {
        let ci = grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, id)| **id == i)
                    .map(|(x, id)| (*id, (x, y)))
                    .collect_vec()
            })
            .collect_vec();

        let touches_sides = ci
            .iter()
            .any(|(_, (x, y))| x == &0 || y == &0 || x == &(max_x - 1) || y == &(max_y - 1));

        if !touches_sides {
            let area = ci.len();
            if area > max_area {
                max_area = area
            }
        }
    }

    max_area
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
