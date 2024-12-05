use std::collections::{ HashMap, HashSet };
use advent_of_code_2024::utils::util::read_lines;

pub fn run(input_path: &str) -> i32 {
    let mut result: i32 = 0;
    if let Ok(lines) = read_lines(input_path) {
        let mut first_part = true;
        let mut has_greater_index: HashMap<i32, HashSet<i32>> = HashMap::new();
        let mut has_lesser_index: HashMap<i32, HashSet<i32>> = HashMap::new();
        for line in lines.flatten() {
            if !first_part {
                //println!("2. part {}", line);
                let v = line
                    .split(",")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<_>>();
                let mut v_map: HashMap<i32, usize> = HashMap::new();
                for (i, e) in v.iter().enumerate() {
                    v_map.insert(*e, i);
                }
                let mut correct = true;
                for i in &v {
                    //print!("{} ", v_map.get(i).unwrap());
                    if has_greater_index.contains_key(i) {
                        let set = has_greater_index.get(i).unwrap();
                        for j in set {
                            if
                                v_map.contains_key(j) &&
                                v_map.get(i).unwrap() > v_map.get(j).unwrap()
                            {
                                correct = false;
                                break;
                            }
                        }
                    }
                    if !correct {
                        break;
                    }
                    if has_lesser_index.contains_key(i) {
                        let set = has_lesser_index.get(i).unwrap();
                        for j in set {
                            if
                                v_map.contains_key(j) &&
                                v_map.get(i).unwrap() < v_map.get(j).unwrap()
                            {
                                correct = false;
                                break;
                            }
                        }
                    }
                    if !correct {
                        break;
                    }
                }
                if correct {
                    result += v.get((v.len() - 1) / 2).unwrap();
                }
            }
            if line.len() == 0 {
                first_part = false;
            }
            if first_part {
                let split = line
                    .split("|")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<_>>();
                if has_greater_index.contains_key(split.get(0).unwrap()) {
                    has_greater_index
                        .get_mut(split.get(0).unwrap())
                        .unwrap()
                        .insert(split.get(1).unwrap().to_owned());
                } else {
                    let mut set = HashSet::new();
                    set.insert(split.get(1).unwrap().to_owned());
                    has_greater_index.insert(split.get(0).unwrap().to_owned(), set);
                }

                if has_lesser_index.contains_key(split.get(1).unwrap()) {
                    has_lesser_index
                        .get_mut(split.get(1).unwrap())
                        .unwrap()
                        .insert(split.get(0).unwrap().to_owned());
                } else {
                    let mut set = HashSet::new();
                    set.insert(split.get(0).unwrap().to_owned());
                    has_lesser_index.insert(split.get(1).unwrap().to_owned(), set);
                }
            }
        }
        // for e in has_greater_index.iter() {
        //     print!("{}: ", e.0);
        //     for i in e.1 {
        //         print!("{} ", i);
        //     }
        //     println!();
        // }
        // println!();
        // for e in has_lesser_index.iter() {
        //     print!("{}: ", e.0);
        //     for i in e.1 {
        //         print!("{} ", i);
        //     }
        //     println!();
        // }
    }
    result
}

#[cfg(test)]
mod runs {
    use super::*;
    #[test]
    fn testinput() {
        assert_eq!(run("./src/day05/testinput.txt"), 143);
    }
    #[test]
    fn input() {
        assert_eq!(run("./src/day05/input.txt"), 4609);
    }
}