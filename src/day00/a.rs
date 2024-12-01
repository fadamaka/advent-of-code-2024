use std::collections::HashMap;
use advent_of_code_2024::utils::util::read_lines;

pub fn run() {
    if let Ok(lines) = read_lines("./src/day00/input.txt") {
        let mut parsed_map: HashMap<String, Vec<u16>> = HashMap::new();
        for line in lines.flatten() {
            let mut it = line.split(':').map(String::from);
            let key = it.next().unwrap();
            let list: Vec<u16> = it
                .next()
                .unwrap()
                .split_off(1)
                .split(" ")
                .map(String::from)
                .map(|x| x.parse::<u16>().unwrap())
                .collect();
            parsed_map.insert(key, list);
        }
        for e in parsed_map {
            println!("key:{} list:{:?}", e.0, e.1);
            println!(
                "result:{}",
                e.1
                    .iter()
                    .copied()
                    .reduce(|a, b| a + b)
                    .unwrap()
            );
        }
    }
}

#[cfg(test)]
mod runs {
    use super::*;
    #[test]
    fn testinput() {
        run();
    }
}
