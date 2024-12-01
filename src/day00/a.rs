use std::fs::File;
use std::io::{ self, BufRead };
use std::path::Path;
use std::collections::HashMap;

pub fn day_00() {
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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod days {
    use super::*;
    #[test]
    fn sda() {
        day_00();
    }
}
