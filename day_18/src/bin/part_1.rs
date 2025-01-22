use itertools::Itertools;
use std::time::Instant;

const TEST_DIMS: usize = 10;
const DIMS: usize = 50;

fn get_neighbours(area: &[[char; DIMS]; DIMS], (row, col): (usize, usize)) -> Vec<char> {
    [
        (row.overflowing_sub(1).0, col.overflowing_sub(1).0),
        (row.overflowing_sub(1).0, col),
        (row.overflowing_sub(1).0, col + 1),
        (row, col.overflowing_sub(1).0),
        (row, col + 1),
        (row + 1, col.overflowing_sub(1).0),
        (row + 1, col),
        (row + 1, col + 1),
    ]
    .into_iter()
    .filter_map(|(nr, nc)| {
        if nr < DIMS && nc < DIMS {
            Some(area[nr][nc])
        } else {
            None
        }
    })
    .collect_vec()
}

fn get_next(cell: char, neighbours: &[char]) -> char {
    match cell {
        '|' => {
            if neighbours.iter().filter(|&c| *c == '#').count() >= 3 {
                '#'
            } else {
                cell
            }
        }
        '#' => {
            if neighbours.contains(&'#') && neighbours.contains(&'|') {
                cell
            } else {
                '.'
            }
        }
        '.' => {
            if neighbours.iter().filter(|&c| *c == '|').count() >= 3 {
                '|'
            } else {
                cell
            }
        }
        _ => unreachable!(),
    }
}

fn debug_area(area: &[[char; DIMS]; DIMS]) {
    println!("\n--------- AREA ----------\n");
    for row in area.iter() {
        for c in row.iter() {
            print!("{c}");
        }
        println!();
    }
    println!("\n-------------------------\n");
}

fn process(input: &str) -> usize {
    let mut area: [[char; DIMS]; DIMS] = input
        .lines()
        .filter_map(|l| l.chars().collect_array())
        .collect_array()
        .expect("should be 10 by 10");

    for i in 0..10 {
        let mut current = [['X'; DIMS]; DIMS];
        for row in 0..DIMS {
            for col in 0..DIMS {
                let neighbours = get_neighbours(&area, (row, col));
                let cell = area[row][col];
                let next = get_next(cell, &neighbours);
                current[row][col] = next;
            }
        }

        area = current;
    }

    let lumberyards: usize = area
        .iter()
        .map(|r| r.iter().filter(|&c| *c == '#').count())
        .sum();

    let trees: usize = area
        .iter()
        .map(|r| r.iter().filter(|&c| *c == '|').count())
        .sum();

    lumberyards * trees
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
