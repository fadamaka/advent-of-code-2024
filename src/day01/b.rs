use advent_of_code_2024::utils::util::read_lines;

pub fn run(input_path: &str) -> i32 {
    let mut result: i32 = 0;
    if let Ok(lines) = read_lines(input_path) {
        let mut left_v: Vec<i32> = Vec::new();
        let mut right_v: Vec<i32> = Vec::new();
        for line in lines.flatten() {
            let it = line.split("   ").map(String::from);
            let nums: Vec<i32> = it.map(|x| x.parse::<i32>().unwrap()).collect();
            left_v.push(*nums.first().unwrap());
            right_v.push(*nums.last().unwrap());
        }
        for i in left_v {
            let newvec = right_v.to_vec();
            let mut counter: i32 = 0;
            for j in newvec {
                if i == j {
                    counter += 1;
                }
            }
            result += i * counter;
        }

        println!("result: {:?}", result);
    }
    result
}

#[cfg(test)]
mod runs {
    use super::*;
    #[test]
    fn input() {
        assert_eq!(run("./src/day01/input.txt"), 19678534);
    }
    #[test]
    fn testinput() {
        assert_eq!(run("./src/day01/testinput.txt"), 31);
    }
}
