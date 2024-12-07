use std::collections::HashMap;

use advent_of_code_2024::utils::util::read_lines;
use itertools::Itertools;

pub fn run(input_path: &str) -> i64 {
    let mut result: i64 = 0;
    if let Ok(lines) = read_lines(input_path) {
        for line in lines.flatten() {
            let mut split = line.split(": ");
            let test_value = split.next().unwrap().parse::<i64>().unwrap();
            let mut nums = split
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            nums.reverse();
            if test_seq(nums, test_value, 0) {
                result += test_value;
            }
        }
    }
    result
}

fn test_seq(nums: Vec<i64>, test_value: i64, start: usize) -> bool {
    let remainder = test_value;
    if remainder > 0 && start < nums.len() - 1 {
        let n = nums[start];
        let mut pt2 = remainder - n;
        for _i in 0..n.to_string().len() {
            if pt2 > 0 && pt2 % 10 == 0 {
                pt2 /= 10;
            }
        }
        if remainder % n == 0 {
            return (
                test_seq(nums.clone(), remainder / n, start + 1) ||
                test_seq(nums.clone(), remainder - n, start + 1) ||
                test_seq(nums.clone(), pt2, start + 1)
            );
        } else {
            return (
                test_seq(nums.clone(), remainder - n, start + 1) ||
                test_seq(nums.clone(), pt2, start + 1)
            );
        }
    }
    remainder == nums[start]
}

#[cfg(test)]
mod runs {
    use super::*;
    #[test]
    fn testinput() {
        assert_eq!(run("./src/day07/testinput.txt"), 11387);
    }
    #[test]
    fn input() {
        assert_eq!(run("./src/day07/input.txt"), 116094961956019);
    }
    #[test]
    fn test2() {
        assert_eq!(run("./src/day07/test2.txt"), 0);
    }
}
