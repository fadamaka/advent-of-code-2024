use advent_of_code_2024::utils::util::read_lines;

pub fn run(input_path: &str) -> i32 {
    let mut result: i32 = 0;
    if let Ok(lines) = read_lines(input_path) {
        let mut distance_v: Vec<i32> = Vec::new();
        let mut left_v: Vec<i32> = Vec::new();
        let mut right_v: Vec<i32> = Vec::new();
        for line in lines.flatten() {
            let nums: Vec<i32> = line
                .split("   ")
                .map(String::from)
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            left_v.push(*nums.first().unwrap());
            right_v.push(*nums.last().unwrap());
        }
        left_v.sort();
        right_v.sort();

        for (pos, e) in left_v.iter().enumerate() {
            distance_v.push((right_v[pos] - e).abs());
        }
        result = distance_v
            .iter()
            .copied()
            .reduce(|a, b| a + b)
            .unwrap();
        println!("result: {:?}", result);
    }
    result
}

#[cfg(test)]
mod runs {
    use super::*;
    #[test]
    fn testinput() {
        assert_eq!(run("./src/day01/testinput.txt"), 11);
    }
    #[test]
    fn input() {
        assert_eq!(run("./src/day01/input.txt"), 2031679);
    }
}
