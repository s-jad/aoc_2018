use itertools::Itertools;
use std::time::Instant;

fn calculate_power_level(x: i32, y: i32, serial: i32) -> i32 {
    let rack_id = x + 10;
    let mut power_level = rack_id * y;
    power_level += serial;
    power_level *= rack_id;
    let hundreds_digit = (power_level / 100) % 10;
    hundreds_digit - 5
}

fn process() -> (i32, i32, i32) {
    const GRID_DIMS: i32 = 300;
    const SERIAL_NUM: i32 = 9221;

    let mut max = std::i32::MIN;
    let mut coords = (0, 0, 0);

    for dims in 1..=10 {
        for y in 1..GRID_DIMS - dims {
            for x in 1..GRID_DIMS - dims {
                let mut total_power = 0;
                for dy in 0..dims {
                    for dx in 0..dims {
                        total_power += calculate_power_level(x + dx, y + dy, SERIAL_NUM);
                    }
                }
                if total_power > max {
                    max = total_power;
                    coords = (x, y, dims);
                }
            }
        }
    }

    coords
}

fn main() {
    let start = Instant::now();
    let output = process();
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
