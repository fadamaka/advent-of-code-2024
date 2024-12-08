use advent_of_code_2024::utils::util::read_lines;

pub fn run(input_path: &str) -> i32 {
    let mut result: i32 = 0;
    if let Ok(lines) = read_lines(input_path) {
        for line in lines.flatten() {
            println!("{}", line);
        }
    }
    result
}

#[cfg(test)]
mod runs {
    use super::*;
    #[test]
    fn testinput() {
        assert_eq!(run("./src/day##/testinput.txt"), 0);
    }
    #[test]
    fn input() {
        assert_eq!(run("./src/day##/input.txt"), 0);
    }
}
