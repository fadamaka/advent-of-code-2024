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
            // print!("{:?}: ", test_value);
            // for num in nums.clone() {
            //     print!("{} ", num);
            // }
            // println!();
            let mut possible = false;

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
    if remainder > 0 && start < nums.len() {
        let n = nums[start];
        if remainder % n == 0 {
            return (
                test_seq(nums.clone(), remainder / n, start + 1) ||
                test_seq(nums.clone(), remainder - n, start + 1)
            );
        } else {
            return test_seq(nums.clone(), remainder - n, start + 1);
        }
    }
    remainder == 0
}

pub fn run_bf(input_path: &str) -> i64 {
    let mut result: i64 = 0;
    if let Ok(lines) = read_lines(input_path) {
        for line in lines.flatten() {
            let mut split = line.split(": ");
            let test_value = split.next().unwrap().parse::<i64>().unwrap();
            let nums = split
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            // print!("{:?}: ", test_value);
            // for num in nums.clone() {
            //     print!("{} ", num);
            // }
            // println!();
            let mut possible = false;

            let mut plus_v = vec!['+'; nums.len()-1];
            let mut operators = vec!['*';nums.len()-1];
            operators.append(&mut plus_v);
            let nums_mul = nums
                .clone()
                .iter()
                .copied()
                .reduce(|a, b| a * b)
                .unwrap();

            let mut cache: HashMap<String, i64> = HashMap::new();
            // println!("{}", test_value);
            // for n in nums.clone() {
            //     print!("{} ", n);
            // }
            // println!();

            if test_value < nums_mul {
                for i in operators
                    .into_iter()
                    .permutations(nums.len() - 1)
                    .unique() {
                    // for c in i.clone() {
                    //     print!("{} ", c);
                    // }
                    // println!();
                    let mut num_it = nums.clone().into_iter();
                    let mut calc_res: i64 = num_it.next().unwrap();
                    let mut key = String::from("");
                    for j in i {
                        key.push(j);
                        if cache.contains_key(&key) {
                            calc_res = cache.get(&key).unwrap().to_owned();
                            num_it.next();
                        } else {
                            if calc_res > test_value {
                                break;
                            }
                            if j == '+' {
                                let add = num_it.next().unwrap();
                                calc_res += add;
                            } else {
                                let mul = num_it.next().unwrap();
                                calc_res *= mul;
                            }
                            cache.insert(key.clone(), calc_res.clone());
                        }
                    }
                    if calc_res == test_value {
                        possible = true;
                        break;
                    }
                }
            } else {
                if test_value == nums_mul {
                    possible = true;
                }
            }

            if possible {
                result += test_value;
            }
        }
    }
    result
}
#[cfg(test)]
mod runs {
    use super::*;
    #[test]
    fn testinput_bf() {
        assert_eq!(run_bf("./src/day07/testinput.txt"), 3749);
    }
    #[test]
    fn testinput() {
        assert_eq!(run("./src/day07/testinput.txt"), 3749);
    }
    #[test]
    fn input() {
        assert_eq!(run("./src/day07/input.txt"), 267566105056);
    }
    #[test]
    fn test2() {
        assert_eq!(run("./src/day07/test2.txt"), run_bf("./src/day07/test2.txt"));
    }
}
