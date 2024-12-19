use std::collections::HashSet;

use advent_of_code_2024::utils::util::read_lines;
use itertools::Itertools;

pub fn run(input_path: &str) -> i32 {
    let mut result: i32 = 0;
    if let Ok(lines) = read_lines(input_path) {
        let mut towels: HashSet<String> = HashSet::new();
        let mut designs: Vec<String> = Vec::new();
        let mut first = true;
        for line in lines.flatten() {
            if line.len() > 0 {
                if first {
                    towels = HashSet::from_iter(line.split(", ").map(|s| String::from(s)));
                    first = false;
                } else {
                    designs.push(line);
                }
            }
        }
        for str in designs {
            if solve(&towels, str) {
                result += 1;
            }
        }
    }
    result
}

fn solve(towels: &HashSet<String>, design: String) -> bool {
    if design.len() == 0 {
        return true;
    }
    let mut subject = String::new();
    let char_v = design.chars().collect_vec();
    for i in 0..char_v.len() {
        subject.push(char_v[i]);
        if towels.contains(&subject) && solve(towels, String::from(design.split_at(i + 1).1)) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod runs {
    use super::*;
    #[test]
    fn testinput() {
        assert_eq!(run("./src/day19/testinput.txt"), 6);
    }
    #[test]
    fn input() {
        assert_eq!(run("./src/day19/input.txt"), 333);
    }
}
