use std::collections::HashMap;

use advent_of_code_2024::utils::util::read_lines;

pub fn run(input_path: &str) -> i32 {
    // mut result: i32 = 0;
    let number_keypad_map = HashMap::from([
        ('A', (3, 2)),
        ('0', (3, 1)),
        ('1', (2, 0)),
        ('2', (2, 1)),
        ('3', (2, 2)),
        ('4', (1, 0)),
        ('5', (1, 1)),
        ('6', (1, 2)),
        ('7', (0, 0)),
        ('8', (0, 1)),
        ('9', (0, 2)),
    ]);
    let keypad_map = HashMap::from([
        ((0, 0), (0, 2)),
        ((-1, 0), (0, 1)),
        ((1, 0), (1, 1)),
        ((0, -1), (1, 0)),
        ((0, 1), (1, 2)),
    ]);
    if let Ok(lines) = read_lines(input_path) {
        for line in lines.flatten() {
            println!("{}", line);
        }
    }
    0
}

#[cfg(test)]
mod runs {
    use super::*;
    #[test]
    fn testinput() {
        assert_eq!(run("./src/day21/testinput.txt"), 0);
    }
    #[test]
    fn input() {
        assert_eq!(run("./src/day21/input.txt"), 0);
    }
}
