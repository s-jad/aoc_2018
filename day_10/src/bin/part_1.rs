use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    let mut points = input
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
        let max_x = points
            .iter()
            .map(|(pos, vel)| pos.0 + vel.0 * i)
            .max()
            .unwrap();
        let min_x = points
            .iter()
            .map(|(pos, vel)| pos.0 + vel.0 * i)
            .min()
            .unwrap();
        let max_y = points
            .iter()
            .map(|(pos, vel)| pos.1 + vel.1 * i)
            .max()
            .unwrap();
        let min_y = points
            .iter()
            .map(|(pos, vel)| pos.1 + vel.1 * i)
            .min()
            .unwrap();

        let area = (max_x.abs_diff(min_x) as usize * max_y.abs_diff(min_y) as usize) as usize;

        if area < min_area.0 {
            min_area = (area, i);
            minmax_x = (min_x, max_x);
            minmax_y = (min_y, max_y);
        }
    }

    println!("");
    for y in minmax_y.0..=minmax_y.1 {
        for x in minmax_x.0..=minmax_x.1 {
            if points
                .iter()
                .map(|(p, v)| (p.0 + v.0 * min_area.1, p.1 + v.1 * min_area.1))
                .contains(&(x, y))
            {
                print!("#");
            } else {
                print!(".")
            }
        }
        println!("")
    }

    // for (pos, vel) in points.iter_mut() {
    //     let nx = pos.0 + vel.0;
    //     let ny = pos.1 + vel.1;
    //     *pos = (nx, ny);
    // }

    // if i % 10 == 0 {
    //     for y in (min_y + 1)..max_y {
    //         for x in (min_x + 1)..max_x {
    //             if points.iter().map(|(pos, _)| pos).contains(&(x, y)) {
    //                 print!("#");
    //             } else {
    //                 print!(".");
    //             }
    //         }
    //         println!("");
    //     }
    //     println!("");
    // }

    1
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
