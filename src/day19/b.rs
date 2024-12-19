use std::{ collections::{ HashMap, HashSet }, sync::{ Mutex, MutexGuard, OnceLock } };

use advent_of_code_2024::utils::util::read_lines;
use itertools::Itertools;

pub fn run(input_path: &str) -> i128 {
    let mut result: i128 = 0;
    if let Ok(lines) = read_lines(input_path) {
        let mut towels: HashSet<String> = HashSet::new();
        let mut designs: Vec<String> = Vec::new();
        let mut first = true;
        for line in lines.flatten() {
            if line.len() > 0 {
                if first {
                    towels = HashSet::from_iter(line.split(", ").map(|s| String::from(s)));
                    first = false;
                } else {
                    designs.push(line);
                }
            }
        }
        for str in designs {
            if solvable(&towels, str.clone()) {
                result += solve(&towels, str);
            }
        }
    }
    result
}

fn solvable(towels: &HashSet<String>, design: String) -> bool {
    if design.len() == 0 {
        return true;
    }
    let mut subject = String::new();
    let char_v = design.chars().collect_vec();
    for i in 0..char_v.len() {
        subject.push(char_v[i]);
        if towels.contains(&subject) && solvable(towels, String::from(design.split_at(i + 1).1)) {
            return true;
        }
    }
    false
}

fn get_global_hashmap() -> MutexGuard<'static, HashMap<String, i128>> {
    static MAP: OnceLock<Mutex<HashMap<String, i128>>> = OnceLock::new();
    MAP.get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .expect("Let's hope the lock isn't poisoned")
}

fn solve(towels: &HashSet<String>, design: String) -> i128 {
    if get_global_hashmap().contains_key(&design) {
        return get_global_hashmap().get(&design).unwrap().to_owned();
    }
    if design.len() == 0 {
        return 1;
    }
    let mut result = 0;
    let mut subject = String::new();
    let char_v = design.chars().collect_vec();
    for i in 0..char_v.len() {
        subject.push(char_v[i]);
        if towels.contains(&subject) {
            result += solve(towels, String::from(design.split_at(i + 1).1));
        }
    }
    get_global_hashmap().insert(design, result);
    result
}

#[cfg(test)]
mod runs {
    use super::*;
    // #[test]
    // fn testinput() {
    //     assert_eq!(run("./src/day19/testinput.txt"), 16);
    // }
    #[test]
    fn input() {
        assert_eq!(run("./src/day19/input.txt"), 678536865274732);
    }
}
