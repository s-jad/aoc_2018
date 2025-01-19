use std::time::Instant;

use itertools::Itertools;

const GOAL: [u8; 6] = [9, 3, 9, 6, 0, 1];
const SLICE_LEN: usize = 6;
const TEST_GOAL: [u8; 5] = [5, 9, 4, 1, 4];

fn extract_digits(num: u8) -> (u8, u8) {
    (num / 10, num % 10)
}

fn process() -> usize {
    let mut recipes = Vec::from([3, 7, 1, 0, 1, 0]);
    let mut elf1_idx = 4;
    let mut elf2_idx = 3;

    loop {
        let r1 = recipes[elf1_idx];
        let r2 = recipes[elf2_idx];
        let new_recipe_sum = r1 + r2;

        let (d1, d2) = extract_digits(new_recipe_sum);

        if d1 != 0 {
            recipes.push(d1);
            let len = recipes.len();
            if recipes[len - SLICE_LEN..] == GOAL {
                return len - SLICE_LEN;
            }
        }

        recipes.push(d2);
        let len = recipes.len();
        if recipes[len - SLICE_LEN..] == GOAL {
            return len - SLICE_LEN;
        }

        elf1_idx = (elf1_idx + (1 + r1) as usize) % len;
        elf2_idx = (elf2_idx + (1 + r2) as usize) % len;
    }
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
        let output = process();
        assert_eq!(result,);
    }
}
