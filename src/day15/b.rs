use std::{ collections::HashMap, sync::Mutex };

use advent_of_code_2024::utils::util::read_lines;
use itertools::Itertools;

static MATRIX: Mutex<Vec<Vec<char>>> = Mutex::new(Vec::new());

pub fn run(input_path: &str) -> usize {
    let mut result: usize = 0;
    let mut dirs: Vec<Vec<char>> = Vec::new();
    let dir_map: HashMap<char, (isize, isize)> = HashMap::from([
        ('^', (-1, 0)),
        ('>', (0, 1)),
        ('v', (1, 0)),
        ('<', (0, -1)),
    ]);

    if let Ok(lines) = read_lines(input_path) {
        let mut first_part = true;
        for line in lines.flatten() {
            if !first_part {
                dirs.push(line.chars().collect_vec());
            }
            if line.len() == 0 {
                first_part = false;
            }
            if first_part {
                MATRIX.lock().unwrap().push(line.chars().collect_vec());
            }
        }
        let dirs_flat = dirs.into_iter().flatten().collect_vec();

        for i in MATRIX.lock().unwrap().clone().into_iter() {
            for j in i {
                print!("{}", j);
            }
            println!();
        }
        let y_max = MATRIX.lock().unwrap().len();
        let x_max = MATRIX.lock().unwrap().first().unwrap().len();
        let mut y = 0;
        let mut x = 0;
        for i in 0..y_max {
            for j in 0..x_max {
                if read(i, j) == '@' {
                    y = i;
                    x = j;
                }
            }
        }
        println!("{} {}", y, x);
        for m in dirs_flat {
            let d = dir_map.get(&m).unwrap();
            let next_tile = (((y as isize) + d.0) as usize, ((x as isize) + d.1) as usize);
            let nt_ch = read(next_tile.0, next_tile.1);
            if nt_ch == '#' {
                continue;
            }
            if nt_ch == '.' {
                write(next_tile.0, next_tile.1, '@');
                write(y, x, '.');
                y = next_tile.0;
                x = next_tile.1;
                continue;
            }
            if nt_ch == 'O' {
                if try_to_push(next_tile, *d) {
                    write(next_tile.0, next_tile.1, '@');
                    write(y, x, '.');
                    y = next_tile.0;
                    x = next_tile.1;
                    continue;
                }
            }
        }
        for i in 0..y_max {
            for j in 0..x_max {
                if read(i, j) == 'O' {
                    result += i * 100 + j;
                }
            }
        }
    }
    for i in MATRIX.lock().unwrap().clone().into_iter() {
        for j in i {
            print!("{}", j);
        }
        println!();
    }

    result
}
fn try_to_push(ct: (usize, usize), d: (isize, isize)) -> bool {
    let next_tile = (((ct.0 as isize) + d.0) as usize, ((ct.1 as isize) + d.1) as usize);
    let nt_ch = read(next_tile.0, next_tile.1);
    if nt_ch == '.' {
        write(next_tile.0, next_tile.1, 'O');
        return true;
    }
    if nt_ch == '#' {
        return false;
    }
    if nt_ch == 'O' {
        return try_to_push(next_tile, d);
    }
    false
}

fn read(y: usize, x: usize) -> char {
    MATRIX.lock().unwrap()[y][x]
}
fn write(y: usize, x: usize, inp: char) {
    MATRIX.lock().unwrap()[y][x] = inp;
}

#[cfg(test)]
mod runs {
    use super::*;
    //#[test]
    fn testinput() {
        assert_eq!(run("./src/day15/testinput.txt"), 10092);
        //*MATRIX.lock().unwrap() = vec![];
    }
    #[test]
    fn input() {
        //let res = MATRIX.lock();
        assert_eq!(run("./src/day15/input.txt"), 1318523);
    }
}
