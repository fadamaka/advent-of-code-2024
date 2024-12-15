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
                MATRIX.lock()
                    .unwrap()
                    .push(
                        line
                            .chars()
                            .map(|ch| {
                                if ch == '.' {
                                    return "..";
                                }
                                if ch == '#' {
                                    return "##";
                                }
                                if ch == '@' {
                                    return "@.";
                                }
                                if ch == 'O' {
                                    return "[]";
                                }
                                "xx"
                            })
                            .join("")
                            .chars()
                            .collect_vec()
                    );
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
                if readbot(i, j) == '@' {
                    y = i;
                    x = j;
                }
            }
        }
        println!("{} {}", y, x);
        for m in dirs_flat {
            // println!("{}", m);
            // print_matrix();
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
            if nt_ch == '[' || nt_ch == ']' {
                if d.0 == 0 {
                    //println!("before hor push try");
                    if try_to_push_sideways(next_tile, *d) {
                        push_sideways(next_tile, *d);
                        write(next_tile.0, next_tile.1, '@');
                        write(y, x, '.');
                        y = next_tile.0;
                        x = next_tile.1;
                        continue;
                    }
                } else {
                    //println!("before vert push try");
                    if try_to_push_vert(next_tile, *d) {
                        //println!("before vert push phys {:?} {:?}", next_tile, *d);
                        //print_matrix();
                        push_vert(next_tile, *d);
                        write(next_tile.0, next_tile.1, '@');
                        write(y, x, '.');
                        y = next_tile.0;
                        x = next_tile.1;
                        continue;
                    }
                }
            }
        }
        for i in 0..y_max {
            for j in 0..x_max {
                if read(i, j) == '[' {
                    result += i * 100 + j;
                }
            }
        }
    }
    print_matrix();
    result
}
fn print_matrix() {
    for i in MATRIX.lock().unwrap().clone().into_iter() {
        for j in i {
            print!("{}", j);
        }
        println!();
    }
}
fn try_to_push_vert(ct: (usize, usize), d: (isize, isize)) -> bool {
    let ct_ch = read(ct.0, ct.1);
    if ct_ch == '.' {
        return true;
    }
    let next_tile = (((ct.0 as isize) + d.0) as usize, ((ct.1 as isize) + d.1) as usize);
    let nt_ch = read(next_tile.0, next_tile.1);
    let p = if ct_ch == '[' { (0, 1) } else { (0, -1) };
    let next_tile_p = (
        ((next_tile.0 as isize) + p.0) as usize,
        ((next_tile.1 as isize) + p.1) as usize,
    );
    let nt_ch_p = read(next_tile_p.0, next_tile_p.1);
    if nt_ch == '.' && nt_ch_p == '.' {
        return true;
    }
    if nt_ch == '#' || nt_ch_p == '#' {
        return false;
    }
    if nt_ch == ct_ch {
        return try_to_push_vert(next_tile, d);
    }
    try_to_push_vert(next_tile, d) && try_to_push_vert(next_tile_p, d)
}

fn push_vert(ct: (usize, usize), d: (isize, isize)) {
    let ct_ch = read(ct.0, ct.1);
    if ct_ch != '[' && ct_ch != ']' {
        return;
    }
    let next_tile = (((ct.0 as isize) + d.0) as usize, ((ct.1 as isize) + d.1) as usize);
    let nt_ch = read(next_tile.0, next_tile.1);
    let p = if ct_ch == '[' { (0, 1) } else { (0, -1) };
    let next_tile_p = (
        ((next_tile.0 as isize) + p.0) as usize,
        ((next_tile.1 as isize) + p.1) as usize,
    );
    let ct_p = (((ct.0 as isize) + p.0) as usize, ((ct.1 as isize) + p.1) as usize);
    let ct_ch_p = read(ct_p.0, ct_p.1);
    let nt_ch_p = read(next_tile_p.0, next_tile_p.1);
    if nt_ch == '.' && nt_ch_p == '.' {
        write(next_tile.0, next_tile.1, ct_ch);
        write(next_tile_p.0, next_tile_p.1, ct_ch_p);

        write(ct.0, ct.1, '.');
        write(ct_p.0, ct_p.1, '.');
    } else {
        push_vert(next_tile, d);
        if ct_ch != nt_ch {
            push_vert(next_tile_p, d);
        }
        write(next_tile.0, next_tile.1, ct_ch);
        write(next_tile_p.0, next_tile_p.1, ct_ch_p);

        write(ct.0, ct.1, '.');
        write(ct_p.0, ct_p.1, '.');
    }
}

fn try_to_push_sideways(ct: (usize, usize), d: (isize, isize)) -> bool {
    let next_tile = (((ct.0 as isize) + d.0) as usize, ((ct.1 as isize) + d.1) as usize);
    let nt_ch = read(next_tile.0, next_tile.1);
    if nt_ch == '.' {
        return true;
    }
    if nt_ch == '#' {
        return false;
    }
    if nt_ch == '[' || nt_ch == ']' {
        return try_to_push_sideways(next_tile, d);
    }
    false
}
fn push_sideways(ct: (usize, usize), d: (isize, isize)) {
    let next_tile = (((ct.0 as isize) + d.0) as usize, ((ct.1 as isize) + d.1) as usize);
    let nt_ch = read(next_tile.0, next_tile.1);
    if nt_ch == '.' {
        write(next_tile.0, next_tile.1, read(ct.0, ct.1));
    } else {
        push_sideways(next_tile, d);
        write(next_tile.0, next_tile.1, read(ct.0, ct.1));
    }
}

fn readbot(y: usize, x: usize) -> char {
    MATRIX.lock().unwrap()[y][x]
}
fn read(y: usize, x: usize) -> char {
    //println!("read at y: {} x: {}", y, x);
    MATRIX.lock().unwrap()[y][x]
}
fn write(y: usize, x: usize, inp: char) {
    MATRIX.lock().unwrap()[y][x] = inp;
}

#[cfg(test)]
mod runs {
    use super::*;
    #[test]
    fn testinput() {
        //assert_eq!(run("./src/day15/testinput.txt"), 9021);
        //*MATRIX.lock().unwrap() = vec![];
    }
    #[test]
    fn input() {
        //let res = MATRIX.lock();
        assert_eq!(run("./src/day15/input.txt"), 1337648);
    }
}
