use std::{ collections::HashSet, sync::Mutex };

use advent_of_code_2024::utils::util::read_lines;
use itertools::Itertools;

static MATRIX: Mutex<Vec<Vec<char>>> = Mutex::new(Vec::new());
static PRIO_MATRIX: Mutex<Vec<Vec<i32>>> = Mutex::new(Vec::new());
static GOOD_POS: Mutex<Vec<(usize, usize)>> = Mutex::new(Vec::new());

pub fn run(input_path: &str) -> i32 {
    let dir = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut result: i32 = 0;
    if let Ok(lines) = read_lines(input_path) {
        for line in lines.flatten() {
            println!("{}", line);
            MATRIX.lock().unwrap().push(line.chars().collect_vec());
            PRIO_MATRIX.lock()
                .unwrap()
                .push(vec![i32::MAX;line.len()]);
        }

        let y_max = MATRIX.lock().unwrap().len();
        let x_max = MATRIX.lock().unwrap().first().unwrap().len();
        let mut y_s = 0;
        let mut x_s = 0;
        let mut y_e = 0;
        let mut x_e = 0;
        for i in 1..y_max - 1 {
            for j in 1..x_max - 1 {
                let mut count = 0;
                if read((i, j)) == '.' {
                    for d in dir.clone() {
                        if
                            read((((i as isize) + d.0) as usize, ((j as isize) + d.1) as usize)) ==
                            '.'
                        {
                            count += 1;
                        }
                    }
                }
                if count > 2 {
                    write((i, j), 'X');
                }
                if read((i, j)) == 'S' {
                    y_s = i;
                    x_s = j;
                }
                if read((i, j)) == 'E' {
                    y_e = i;
                    x_e = j;
                    write((i, j), '.');
                }
            }
        }
        prio_write((y_s, x_s), 0);
        for d in dir {
            let nt = (((y_s as isize) + d.0) as usize, ((x_s as isize) + d.1) as usize);
            if read(nt) == '.' {
                let mut prio = 1;
                if d != (0, 1) {
                    prio += 1000;
                }
                prio_write(nt, prio);
                mov(nt, d);
            }
        }
        print_matrix();
        println!("seats: {}", find_good_seat((y_e, x_e), (0, 0)).len());
        let set: HashSet<(usize, usize)> = GOOD_POS.lock().unwrap().clone().into_iter().collect();
        println!("{}", set.len());
        for ct in set.clone() {
            write(ct, 'O');
        }
        print_matrix();
        println!("{:?} {}", (7, 4), prio_read((7, 4)));
        println!("{:?} {}", (7, 5), prio_read((7, 5)));
        result = set.len() as i32;
    }
    result
}
fn find_good_seat(ct: (usize, usize), prev: (usize, usize)) -> HashSet<(usize, usize)> {
    let ct_ch = read(ct);
    let mut set: HashSet<(usize, usize)> = HashSet::new();
    GOOD_POS.lock().unwrap().push(ct);
    let ct_prio = prio_read(ct);
    for d_f in vec![(-1, 0), (0, 1), (1, 0), (0, -1)] {
        let next_tile_x = (((ct.0 as isize) + d_f.0) as usize, ((ct.1 as isize) + d_f.1) as usize);
        if
            prio_read(next_tile_x) == ct_prio - 1001 ||
            prio_read(next_tile_x) == ct_prio - 1 ||
            prio_read(prev) - 2 == prio_read(next_tile_x)
        {
            for e in find_good_seat(next_tile_x, ct) {
                set.insert(e);
            }
        }
    }
    if set.len() > 0 || ct_ch == 'S' {
        set.insert(ct);
        return set;
    }
    HashSet::new()
}
fn mov(ct: (usize, usize), d: (isize, isize)) {
    //println!("move: {:?}", ct);
    let ct_prio = prio_read(ct);
    let ct_ch = read(ct);
    let next_tile = (((ct.0 as isize) + d.0) as usize, ((ct.1 as isize) + d.1) as usize);
    let nt_ch = read(next_tile);
    if (nt_ch == '.' || nt_ch == 'X') && ct_prio + 1 < prio_read(next_tile) {
        prio_write(next_tile, ct_prio + 1);
        mov(next_tile, d);
    }
    if ct_ch == 'X' || nt_ch == '#' {
        for d_f in vec![(-1, 0), (0, 1), (1, 0), (0, -1)] {
            if d_f != d {
                let next_tile_x = (
                    ((ct.0 as isize) + d_f.0) as usize,
                    ((ct.1 as isize) + d_f.1) as usize,
                );
                if
                    (read(next_tile_x) == '.' || read(next_tile_x) == 'X') &&
                    ct_prio + 1001 < prio_read(next_tile_x)
                {
                    prio_write(next_tile_x, ct_prio + 1001);
                    mov(next_tile_x, d_f);
                }
            }
        }
    }
}

fn read(p: (usize, usize)) -> char {
    MATRIX.lock().unwrap()[p.0][p.1]
}
fn write(p: (usize, usize), inp: char) {
    MATRIX.lock().unwrap()[p.0][p.1] = inp;
}
fn prio_read(p: (usize, usize)) -> i32 {
    PRIO_MATRIX.lock().unwrap()[p.0][p.1]
}
fn prio_write(p: (usize, usize), n: i32) {
    PRIO_MATRIX.lock().unwrap()[p.0][p.1] = n;
}
fn print_matrix() {
    for i in MATRIX.lock().unwrap().clone().into_iter() {
        for j in i {
            print!("{}", j);
        }
        println!();
    }
}

#[cfg(test)]
mod runs {
    use super::*;
    // #[test]
    // fn testinput() {
    //     assert_eq!(run("./src/day16/testinput.txt"), 64);
    // }
    #[test]
    fn input() {
        assert_eq!(run("./src/day16/input.txt"), 494);
    }
}
