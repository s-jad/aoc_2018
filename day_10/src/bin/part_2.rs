use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    let points = input
        .lines()
        .map(|l| {
            let p = l
                .split_terminator(&[',', '<', '>'][..])
                .filter_map(|s| s.trim().parse::<i32>().ok())
                .collect_tuple::<(_, _, _, _)>()
                .unwrap();

            ((p.0, p.1), (p.2, p.3))
        })
        .collect_vec();

    let mut min_area = (std::usize::MAX, 0);
    let mut minmax_x = (0, 0);
    let mut minmax_y = (0, 0);

    for i in 0..11_000 {
        let (min_x, max_x, min_y, max_y) = points
            .iter()
            .map(|(p, v)| (p.0 + v.0 * i, p.1 + v.1 * i))
            .fold(
                (std::i32::MAX, std::i32::MIN, std::i32::MAX, std::i32::MIN),
                |acc, pos| {
                    (
                        acc.0.min(pos.0),
                        acc.1.max(pos.0),
                        acc.2.min(pos.1),
                        acc.3.max(pos.1),
                    )
                },
            );
        let area = (max_x.abs_diff(min_x) as usize * max_y.abs_diff(min_y) as usize) as usize;

        if area < min_area.0 {
            min_area = (area, i);
            minmax_x = (min_x, max_x);
            minmax_y = (min_y, max_y);
        }
    }

    min_area.1 as usize
}

fn main() {
    let input = include_str!("../../input.txt");

    let start = Instant::now();
    let output = process(input);
    let time = start.elapsed();

    dbg!(output, time);
}
