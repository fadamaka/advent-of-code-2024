use std::collections::{ HashMap, HashSet };

use advent_of_code_2024::utils::util::read_lines;
use itertools::Itertools;

pub fn run(input_path: &str) -> HashSet<(i32, i32)> {
    let mut anti_nodes: HashSet<(i32, i32)> = HashSet::new();
    if let Ok(lines) = read_lines(input_path) {
        let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
        let mut y_bound = 0;
        let mut x_bound = 0;
        for (i, line) in lines.flatten().enumerate() {
            y_bound = i;
            if x_bound == 0 {
                x_bound = line.len() - 1;
            }
            //println!("{}: {}", i, line);
            for (j, ch) in line.chars().enumerate() {
                if ch == '.' {
                    continue;
                }
                if antennas.contains_key(&ch) {
                    antennas
                        .get_mut(&ch)
                        .unwrap()
                        .push((i.try_into().unwrap(), j.try_into().unwrap()));
                } else {
                    antennas.insert(ch, vec![(i.try_into().unwrap(), j.try_into().unwrap())]);
                }
            }
        }
        antennas.into_iter().for_each(|(k, v)| {
            // if k == '#' {
            //     println!("{}: {:?}", k, v);
            // }
            for e in v.into_iter().combinations(2) {
                if e.len() == 2 {
                    let a = e.get(0).unwrap();
                    let b = e.get(1).unwrap();
                    let diff: (i32, i32) = (
                        a.0.abs_diff(b.0).try_into().unwrap(),
                        a.1.abs_diff(b.1).try_into().unwrap(),
                    );
                    //a->b->c
                    let y = if a.0 < b.0 { diff.0 * -1 } else { diff.0 };
                    let x = if a.1 < b.1 { diff.1 * -1 } else { diff.1 };
                    let mut y_pos = a.0;
                    let mut x_pos = a.1;
                    while
                        y_pos >= 0 &&
                        y_pos <= y_bound.try_into().unwrap() &&
                        x_pos >= 0 &&
                        x_pos <= x_bound.try_into().unwrap()
                    {
                        anti_nodes.insert((y_pos, x_pos));
                        y_pos += y;
                        x_pos += x;
                    }
                    let mut y_pos = a.0;
                    let mut x_pos = a.1;
                    while
                        y_pos >= 0 &&
                        y_pos <= y_bound.try_into().unwrap() &&
                        x_pos >= 0 &&
                        x_pos <= x_bound.try_into().unwrap()
                    {
                        anti_nodes.insert((y_pos, x_pos));
                        y_pos += y * -1;
                        x_pos += x * -1;
                    }
                }
            }
        });
        // for i in anti_nodes.clone() {
        //     println!("{:?}", i);
        // }
    }
    anti_nodes
}

#[cfg(test)]
mod runs {
    use super::*;
    #[test]
    fn testinput() {
        assert_eq!(run("./src/day08/testinput.txt").len(), 34)
    }
    #[test]
    fn input() {
        assert_eq!(run("./src/day08/input.txt").len(), 332)
    }
}
