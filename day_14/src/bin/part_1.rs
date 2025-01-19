use std::time::Instant;

const INPUT: usize = 939601;
const SCORING_RECIPES: usize = 10;
const END: usize = INPUT + SCORING_RECIPES;
const RECIPE_ARR_LEN: usize = END + (END / 2);

fn extract_digits(num: u8) -> (u8, u8) {
    (num / 10, num % 10)
}

fn process() -> String {
    let mut recipes = [0u8; RECIPE_ARR_LEN];
    recipes[0] = 3;
    recipes[1] = 7;

    let mut elf1_idx = 0;
    let mut elf2_idx = 1;
    let mut recipe_idx = 2;

    for i in 0..END {
        let r1 = recipes[elf1_idx];
        let r2 = recipes[elf2_idx];
        let new_recipe_sum = r1 + r2;

        let (d1, d2) = extract_digits(new_recipe_sum);
        if d1 != 0 {
            recipes[recipe_idx] = d1;
            recipe_idx += 1;
        }
        recipes[recipe_idx] = d2;
        recipe_idx += 1;

        elf1_idx = (elf1_idx + (1 + r1) as usize) % recipe_idx;
        elf2_idx = (elf2_idx + (1 + r2) as usize) % recipe_idx;
    }

    recipes[INPUT..END]
        .iter()
        .map(|&n| (n + 48) as char)
        .collect::<String>()
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
