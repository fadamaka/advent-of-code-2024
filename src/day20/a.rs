use std::sync::Mutex;
use advent_of_code_2024::utils::util::read_lines;
use itertools::Itertools;

static MATRIX: Mutex<Vec<Vec<char>>> = Mutex::new(Vec::new());
static PRIO_MATRIX: Mutex<Vec<Vec<i32>>> = Mutex::new(Vec::new());
static GRID_SIZE: Mutex<isize> = Mutex::new(0);
static VISITED_V: Mutex<Vec<(isize, isize)>> = Mutex::new(Vec::new());
pub fn run(input_path: &str) -> i32 {
    let mut result: i32 = 0;
    if let Ok(lines) = read_lines(input_path) {
        for line in lines.flatten() {
            if *GRID_SIZE.lock().unwrap() == 0 {
                *GRID_SIZE.lock().unwrap() = line.len() as isize;
            }
            MATRIX.lock().unwrap().push(line.chars().collect_vec());
            PRIO_MATRIX.lock()
                .unwrap()
                .push(vec![i32::MAX;line.len()]);
        }

        let mut y_s = 0;
        let mut x_s = 0;
        let mut y_e = 0;
        let mut x_e = 0;
        let grid_size = *GRID_SIZE.lock().unwrap();
        for i in 0..grid_size {
            for j in 0..grid_size {
                if read((i, j)) == 'S' {
                    y_s = i;
                    x_s = j;
                    prio_write((i, j), 0);
                }
                if read((i, j)) == 'E' {
                    y_e = i;
                    x_e = j;
                    write((i, j), '.');
                }
            }
        }

        let mut ct = (y_s, x_s);
        while prio_read((y_e, x_e)) == i32::MAX {
            VISITED_V.lock().unwrap().push(ct);
            //println!("move: {:?}", ct);
            let ct_prio = prio_read(ct);

            for d_f in vec![(-1, 0), (0, 1), (1, 0), (0, -1)] {
                let next_tile_x = ((ct.0 as isize) + d_f.0, (ct.1 as isize) + d_f.1);
                if read(next_tile_x) == '.' && ct_prio + 1 < prio_read(next_tile_x) {
                    prio_write(next_tile_x, ct_prio + 1);
                    ct = next_tile_x;
                    break;
                }
            }
        }

        for p in VISITED_V.lock().unwrap().clone() {
            let ct_prio = prio_read(p);
            for d_f in vec![(-2, 0), (0, 2), (2, 0), (0, -2)] {
                let next_tile_x = ((p.0 as isize) + d_f.0, (p.1 as isize) + d_f.1);
                if prio_read(next_tile_x) != i32::MAX && prio_read(next_tile_x) - 2 > ct_prio {
                    //println!("shortcut: {}", prio_read(next_tile_x) - ct_prio - 2);
                    if prio_read(next_tile_x) - ct_prio - 2 > 99 {
                        result += 1;
                    }
                }
            }
        }
        //result = prio_read((y_e, x_e));
    }
    result
}

fn mov(ct: (isize, isize)) {
    VISITED_V.lock().unwrap().push(ct);
    //println!("move: {:?}", ct);
    let ct_prio = prio_read(ct);

    for d_f in vec![(-1, 0), (0, 1), (1, 0), (0, -1)] {
        let next_tile_x = ((ct.0 as isize) + d_f.0, (ct.1 as isize) + d_f.1);
        if read(next_tile_x) == '.' && ct_prio + 1 < prio_read(next_tile_x) {
            prio_write(next_tile_x, ct_prio + 1);
            mov(next_tile_x);
        }
    }
}

fn read(p: (isize, isize)) -> char {
    if p.0 > *GRID_SIZE.lock().unwrap() - 1 || p.1 > *GRID_SIZE.lock().unwrap() - 1 {
        return '#';
    }
    if p.0 < 0 || p.1 < 0 {
        return '#';
    }
    MATRIX.lock().unwrap()[p.0 as usize][p.1 as usize]
}
fn write(p: (isize, isize), inp: char) {
    MATRIX.lock().unwrap()[p.0 as usize][p.1 as usize] = inp;
}
fn prio_read(p: (isize, isize)) -> i32 {
    if p.0 > *GRID_SIZE.lock().unwrap() - 1 || p.1 > *GRID_SIZE.lock().unwrap() - 1 {
        return 0;
    }
    if p.0 < 0 || p.1 < 0 {
        return 0;
    }
    PRIO_MATRIX.lock().unwrap()[p.0 as usize][p.1 as usize]
}
fn prio_write(p: (isize, isize), n: i32) {
    PRIO_MATRIX.lock().unwrap()[p.0 as usize][p.1 as usize] = n;
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
    #[test]
    #[serial]
    fn testinput() {
        assert_eq!(run("./src/day20/testinput.txt"), 0);
    }
    #[test]
    #[serial]
    fn input() {
        assert_eq!(run("./src/day20/input.txt"), 1395);
    }
}
