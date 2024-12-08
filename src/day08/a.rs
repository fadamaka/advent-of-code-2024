use std::collections::HashMap;

use advent_of_code_2024::utils::util::read_lines;

pub fn run(input_path: &str) -> i32 {
    let mut result: i32 = 0;
    if let Ok(lines) = read_lines(input_path) {
        let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
        for (i, line) in lines.flatten().enumerate() {
            println!("{}: {}", i, line);
            for (j, ch) in line.chars().enumerate() {
                if ch == '.' {
                    continue;
                }
                if antennas.contains_key(&ch) {
                    antennas.get_mut(&ch).unwrap().push((i, j));
                } else {
                    antennas.insert(ch, vec![(i, j)]);
                }
            }
        }
        antennas.into_iter().for_each(|(k, v)| {
            for e in v {
                println!("{}: {:?}", k, e);
            }
        });
    }
    result
}

#[cfg(test)]
mod runs {
    use super::*;
    #[test]
    fn testinput() {
        assert_eq!(run("./src/day08/testinput.txt"), 0);
    }
    #[test]
    fn input() {
        assert_eq!(run("./src/day08/input.txt"), 0);
    }
}
