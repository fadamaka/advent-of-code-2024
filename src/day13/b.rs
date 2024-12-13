use std::f64;
use advent_of_code_2024::utils::util::read_lines;
use regex::Regex;

pub fn run(input_path: &str) -> i128 {
    let mut result: i128 = 0;
    if let Ok(lines) = read_lines(input_path) {
        let mut a = 0.0 as f64;
        let mut b = 0.0 as f64;
        let mut c = 0.0 as f64;
        let mut d = 0.0 as f64;
        for line in lines.flatten() {
            if line.len() > 0 {
                let re: Regex = Regex::new(r"\d+").unwrap();
                let nums: Vec<_> = line
                    .match_indices(&re)
                    .map(|x| x.1.parse::<f64>().unwrap())
                    .collect();
                if line.find("Button A").is_some() {
                    a = nums[0];
                    c = nums[1];
                    continue;
                }
                if line.find("Button B").is_some() {
                    b = nums[0];
                    d = nums[1];
                    continue;
                }
                if line.find("Prize").is_some() {
                    let res = solve(
                        a,
                        b,
                        nums[0] + "10000000000000".parse::<f64>().unwrap(),
                        c,
                        d,
                        nums[1] + "10000000000000".parse::<f64>().unwrap()
                    );
                    result += (res.0 as i128) * 3;
                    result += res.1 as i128;
                }
            }
        }
    }
    result
}

fn solve(a: f64, b: f64, e: f64, c: f64, d: f64, f: f64) -> (f64, f64) {
    let determinant = a * d - b * c;
    let x = (e * d - b * f) / determinant;
    let y = (a * f - e * c) / determinant;
    if x.fract() == 0.0 && y.fract() == 0.0 {
        return (x, y);
    }
    (0.0, 0.0)
}

#[cfg(test)]
mod runs {
    use super::*;
    #[test]
    fn testinput() {
        assert_eq!(run("./src/day13/testinput.txt"), 875318608908);
    }
    #[test]
    fn input() {
        assert_eq!(run("./src/day13/input.txt"), 93217456941970);
    }
}
