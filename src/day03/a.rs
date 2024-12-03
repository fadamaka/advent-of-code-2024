use advent_of_code_2024::utils::util::read_lines;
use regex::Regex;

pub fn run(input_path: &str) -> i64 {
    let mut result: i64 = 0;
    if let Ok(lines) = read_lines(input_path) {
        for line in lines.flatten() {
            let re: Regex = Regex::new(r"mul\(\d+,\d+\)").unwrap();
            let re2: Regex = Regex::new(r"\d+").unwrap();

            result += line
                .as_str()
                .match_indices(&re)
                .map(|y|
                    y.1
                        .match_indices(&re2)
                        .map(|x| x.1.parse::<i64>().unwrap())
                        .reduce(|a, b| a * b)
                        .unwrap()
                )
                .reduce(|a, b| a + b)
                .unwrap();
        }
    }
    result
}

#[cfg(test)]
mod runs {
    use super::*;
    #[test]
    fn testinput() {
        assert_eq!(run("./src/day03/testinput.txt"), 161);
    }
    #[test]
    fn input() {
        assert_eq!(run("./src/day03/input.txt"), 173529487);
    }
}
