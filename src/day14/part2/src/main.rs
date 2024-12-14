use std::fs::File;
use std::io::{ self, BufRead };
use std::path::Path;
use regex::Regex;

fn main() {
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for _i in 0..103 {
        let line = vec!['.'; 101];
        matrix.push(line);
    }
    //for i in 0..100000 {
    //print!("{}[2J", 27 as char);
    //println!("{}", i);
    run(matrix.clone(), 8280);
    //}
}

pub fn run(matrix_i: Vec<Vec<char>>, seconds: i32) {
    let mut matrix = matrix_i.clone();
    let x_c = 101;
    let y_c = 103;

    let re: Regex = Regex::new(r"-?\d+").unwrap();
    if let Ok(lines) = read_lines("src/input.txt") {
        for line in lines.flatten() {
            let nums: Vec<_> = line
                .match_indices(&re)
                .map(|x| x.1.parse::<i32>().unwrap())
                .collect();

            let x = nums[0];
            let y = nums[1];
            let x_v = nums[2];
            let y_v = nums[3];
            let x_u = (x + x_v * seconds) % x_c;
            let y_u = (y + y_v * seconds) % y_c;
            let x_r = if x_u < 0 { x_u + x_c } else { x_u };
            let y_r = if y_u < 0 { y_u + y_c } else { y_u };

            matrix[y_r as usize][x_r as usize] = '0';
        }
    }
    let mut count = 0;
    let mut found = false;
    for i in matrix.clone() {
        for j in i {
            if j == '0' {
                count += 1;
            }
        }
        if count > 25 {
            found = true;
            break;
        } else {
            count = 0;
        }
    }
    if found {
        println!("{}", seconds);
        for i in matrix {
            for j in i {
                print!("{}", j);
            }
            println!();
        }
    }
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
