use advent_of_code_2024::utils::util::read_lines;

pub fn run(input_path: &str) -> i32 {
    let mut result: i32 = 0;
    if let Ok(lines) = read_lines(input_path) {
        let lines_v = lines.flatten().collect::<Vec<_>>();
        for i in 0..lines_v.len() {
            let line = lines_v.get(i).unwrap();
            for j in 0..line.len() {
                if line.chars().nth(j).unwrap() == 'A' {
                    if find_x(i, j, lines_v.to_owned()) {
                        result += 1;
                    }
                }
            }
        }
    }
    result
}

fn find_x(i: usize, j: usize, lines: Vec<String>) -> bool {
    let mut axis_1: bool = false;
    let mut axis_2: bool = false;
    if j == 0 || i == 0 {
        return false;
    }
    let above_line: Vec<_> = lines
        .get(i - 1)
        .unwrap_or(&String::from("NOT IT"))
        .chars()
        .collect();
    let below_line: Vec<_> = lines
        .get(i + 1)
        .unwrap_or(&String::from("NOT IT"))
        .chars()
        .collect();
    if
        above_line.get(j - 1).unwrap_or(&'B') == &'M' &&
        below_line.get(j + 1).unwrap_or(&'B') == &'S'
    {
        axis_1 = true;
    }
    if
        above_line.get(j - 1).unwrap_or(&'B') == &'S' &&
        below_line.get(j + 1).unwrap_or(&'B') == &'M'
    {
        axis_1 = true;
    }

    if
        above_line.get(j + 1).unwrap_or(&'B') == &'M' &&
        below_line.get(j - 1).unwrap_or(&'B') == &'S'
    {
        axis_2 = true;
    }
    if
        above_line.get(j + 1).unwrap_or(&'B') == &'S' &&
        below_line.get(j - 1).unwrap_or(&'B') == &'M'
    {
        axis_2 = true;
    }

    axis_1 && axis_2
}

#[cfg(test)]
mod runs {
    use super::*;
    #[test]
    fn testinput() {
        assert_eq!(run("./src/day04/testinput.txt"), 9);
    }
    #[test]
    fn input() {
        assert_eq!(run("./src/day04/input.txt"), 2029);
    }
}
