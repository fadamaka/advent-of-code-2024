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
            let mut failedAt = 0;
            for n in 0..=nums.len() - 2 {
                failedAt = n;
                let mut negCount = 0;
                let mut pozCount = 0;
                let mut change = nums[n + 1] - nums[n];
                if change > 0 {
                    pozCount += 1;
                } else {
                    negCount += 1;
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
                if poz && negCount > 0 {
                    safe = false;
                    break;
                }
                if neg && pozCount > 0 {
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
            } else {
                let mut newvec = nums.to_vec();
                newvec.remove(failedAt);
                if retry(newvec) {
                    result += 1;
                } else {
                    let mut newvec = nums.to_vec();
                    newvec.remove(failedAt + 1);
                    if retry(newvec) {
                        result += 1;
                    } else {
                        if failedAt == 1 {
                            let mut newvec = nums.to_vec();
                            newvec.remove(0);
                            if retry(newvec) {
                                result += 1;
                            }
                        }
                    }
                }
            }
            //println!("{} safe: {:?}", line, safe);
        }
    }
    result
}

fn retry(nums: Vec<i32>) -> bool {
    let mut safe = true;
    let mut neg = false;
    let mut poz = false;
    for n in 0..=nums.len() - 2 {
        let mut negCount = 0;
        let mut pozCount = 0;
        let mut change = nums[n + 1] - nums[n];
        if change > 0 {
            pozCount += 1;
        } else {
            negCount += 1;
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
        if poz && negCount > 0 {
            safe = false;
            break;
        }
        if neg && pozCount > 0 {
            safe = false;
            break;
        }
        if change == 0 {
            safe = false;
            break;
        }
    }
    safe
}

#[cfg(test)]
mod runs {
    use super::*;
    #[test]
    fn testinput() {
        assert_eq!(run("./src/day02/testinput.txt"), 4);
    }
    #[test]
    fn input() {
        assert_eq!(run("./src/day02/input.txt"), 308);
    }
}
