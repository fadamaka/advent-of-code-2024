use advent_of_code_2024::utils::util::read_lines;

pub fn run(input_path: &str) -> i32 {
    let mut result: i32 = 0;
    if let Ok(lines) = read_lines(input_path) {
        let lines_v = lines.flatten().collect::<Vec<_>>();
        for i in 0..lines_v.len() {
            let line = lines_v.get(i).unwrap();
            for j in 0..line.len() {
                if line.chars().nth(j).unwrap() == 'X' {
                    if find_ch_h(i, j, lines_v.to_owned(), "XMAS") {
                        result += 1;
                    }
                    if find_ch_v(i, j, lines_v.to_owned(), "XMAS") {
                        result += 1;
                    }
                    if find_ch_d(i, j, lines_v.to_owned(), "XMAS") {
                        result += 1;
                    }
                    if find_ch_d2(i, j, lines_v.to_owned(), "XMAS") {
                        result += 1;
                    }
                }
                if line.chars().nth(j).unwrap() == 'S' {
                    if find_ch_h(i, j, lines_v.to_owned(), "SAMX") {
                        result += 1;
                    }
                    if find_ch_v(i, j, lines_v.to_owned(), "SAMX") {
                        result += 1;
                    }
                    if find_ch_d(i, j, lines_v.to_owned(), "SAMX") {
                        result += 1;
                    }
                    if find_ch_d2(i, j, lines_v.to_owned(), "SAMX") {
                        result += 1;
                    }
                }
            }
        }
    }
    result
}

fn find_ch_h(i: usize, j: usize, lines: Vec<String>, target: &str) -> bool {
    let line = lines.get(i).unwrap();
    let target_chars: Vec<_> = target.chars().collect();
    let chars: Vec<_> = line.chars().collect();
    for n in 0..target.len() {
        if target_chars.get(0 + n).is_none() {
            println!("WTF");
        }
        if chars.get(j + n).unwrap_or(&'B') != target_chars.get(n).unwrap() {
            return false;
        }
    }
    true
}
fn find_ch_v(i: usize, j: usize, lines: Vec<String>, target: &str) -> bool {
    let target_chars: Vec<_> = target.chars().collect();
    for n in 0..target.len() {
        let chars: Vec<_> = lines
            .get(i + n)
            .unwrap_or(&String::from("NOT IT"))
            .chars()
            .collect();
        if chars.get(j).unwrap_or(&'B') != target_chars.get(n).unwrap() {
            return false;
        }
    }
    true
}
fn find_ch_d(i: usize, j: usize, lines: Vec<String>, target: &str) -> bool {
    let target_chars: Vec<_> = target.chars().collect();
    for n in 0..target.len() {
        let chars: Vec<_> = lines
            .get(i + n)
            .unwrap_or(&String::from("NOT IT"))
            .chars()
            .collect();
        if chars.get(j + n).unwrap_or(&'B') != target_chars.get(n).unwrap() {
            return false;
        }
    }
    true
}
fn find_ch_d2(i: usize, j: usize, lines: Vec<String>, target: &str) -> bool {
    let target_chars: Vec<_> = target.chars().collect();
    for n in 0..target.len() {
        let chars: Vec<_> = lines
            .get(i + n)
            .unwrap_or(&String::from("NOT IT"))
            .chars()
            .collect();
        if j < n || chars.get(j - n).unwrap_or(&'B') != target_chars.get(n).unwrap() {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod runs {
    use super::*;
    #[test]
    fn testinput() {
        assert_eq!(run("./src/day04/testinput.txt"), 18);
    }
    #[test]
    fn input() {
        assert_eq!(run("./src/day04/input.txt"), 2567);
    }
}
