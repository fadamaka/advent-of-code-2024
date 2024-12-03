use advent_of_code_2024::utils::util::read_lines;
use regex::Regex;

pub fn run(input_path: &str) -> i64 {
    let mut result: i64 = 0;
    if let Ok(lines) = read_lines(input_path) {
        let re: Regex = Regex::new(r"(mul\(\d+,\d+\))|(don't\(\))|(do\(\))").unwrap();
        let re2: Regex = Regex::new(r"\d+").unwrap();
        let results = lines.flatten().flat_map(|y|
            y
                .match_indices(&re)
                .map(|x| String::from(x.1))
                .collect::<Vec<_>>()
        );
        let mut do_or_not: i64 = 1;
        for hit in results {
            if hit == "do()" {
                do_or_not = 1;
                continue;
            }
            if hit == "don't()" {
                do_or_not = 0;
                continue;
            }
            let value = hit
                .match_indices(&re2)
                .map(|x| x.1.parse::<i64>().unwrap())
                .reduce(|a, b| a * b)
                .unwrap();
            result += do_or_not * value;
        }

        // for line in lines.flatten() {
        //     for hit in line.match_indices(&re).map(|x| x.1) {
        //         println!("{}", hit);
        //     }
        // }
    }
    result
}

#[cfg(test)]
mod runs {
    use super::*;
    #[test]
    fn testinput() {
        assert_eq!(run("./src/day03/testinput2.txt"), 48);
    }
    #[test]
    fn input() {
        assert_eq!(run("./src/day03/input.txt"), 99532691);
    }
}
