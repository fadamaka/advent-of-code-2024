use advent_of_code_2024::utils::util::read_lines;

pub fn run(input_path: &str) -> i32 {
    let mut result: i32 = 0;
    if let Ok(lines) = read_lines(input_path) {
        for line in lines.flatten() {
            let nums: Vec<i32> = line
                .split(" ")
                .map(String::from)
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            let mut safe = true;
            let mut neg = false;
            let mut poz = false;
            for n in 0..=nums.len() - 2 {
                let mut neg_count = 0;
                let mut poz_count = 0;
                let change = nums[n + 1] - nums[n];
                if change > 0 {
                    poz_count += 1;
                } else {
                    neg_count += 1;
                }
                if n == 0 {
                    if change > 0 {
                        poz = true;
                    } else {
                        neg = true;
                    }
                }
                if change.abs() > 3 {
                    safe = false;
                    break;
                }
                if poz && neg_count > 0 {
                    safe = false;
                    break;
                }
                if neg && poz_count > 0 {
                    safe = false;
                    break;
                }
                if change == 0 {
                    safe = false;
                    break;
                }
            }
            if safe {
                result += 1;
            }
            //println!("{} safe: {:?}", line, safe);
        }
    }
    result
}

#[cfg(test)]
mod runs {
    use super::*;
    #[test]
    fn testinput() {
        assert_eq!(run("./src/day02/testinput.txt"), 2);
    }
    #[test]
    fn input() {
        assert_eq!(run("./src/day02/input.txt"), 236);
    }
}
