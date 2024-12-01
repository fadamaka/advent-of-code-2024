use std::fs::File;
use std::io::{ self, BufRead };
use std::path::Path;

pub fn day_01() {
    if let Ok(lines) = read_lines("./src/day01/testinput.txt") {
        let mut left_v: Vec<i32> = Vec::new();
        let mut right_v: Vec<i32> = Vec::new();
        for line in lines.flatten() {
            let it = line.split("   ").map(String::from);
            let nums: Vec<i32> = it.map(|x| x.parse::<i32>().unwrap()).collect();
            left_v.push(*nums.first().unwrap());
            right_v.push(*nums.last().unwrap());
        }
        let mut result: i32 = 0;
        for i in left_v {
            let newvec = right_v.to_vec();
            let mut counter: i32 = 0;
            for j in newvec {
                if i == j {
                    counter += 1;
                }
            }
            result += i * counter;
        }

        println!("result:{:?}", result);
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
        day_01();
    }
}
