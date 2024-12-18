use std::sync::Mutex;
use advent_of_code_2024::utils::util::read_lines;
use itertools::Itertools;

static MATRIX: Mutex<Vec<Vec<char>>> = Mutex::new(Vec::new());
static PRIO_MATRIX: Mutex<Vec<Vec<i32>>> = Mutex::new(Vec::new());
static GRID_SIZE: Mutex<isize> = Mutex::new(0);
pub fn run(input_path: &str, grid_size: usize, fallen: usize) -> i32 {
    *GRID_SIZE.lock().unwrap() = grid_size as isize;
    let mut result: i32 = 0;
    if let Ok(lines) = read_lines(input_path) {
        let mut coords_v: Vec<(isize, isize)> = Vec::new();
        for line in lines.flatten() {
            let temp = line
                .split(",")
                .map(|x| x.parse::<isize>().unwrap())
                .collect_vec();
            coords_v.push((temp[1], temp[0]));
        }
        for _i in 0..grid_size + 1 {
            MATRIX.lock()
                .unwrap()
                .push(vec!['.';grid_size+1]);
            PRIO_MATRIX.lock()
                .unwrap()
                .push(vec![i32::MAX;grid_size+1]);
        }
        for i in 0..fallen {
            write(coords_v[i], '#');
        }

        let y_s = 0;
        let x_s = 0;
        let y_e = grid_size;
        let x_e = grid_size;
        prio_write((y_s, x_s), 0);
        mov((y_s, x_s));

        print_matrix();
        result = prio_read((y_e as isize, x_e as isize));
        println!("result: {:?}", (coords_v[fallen].1, coords_v[fallen].0));
    }
    result
}

fn mov(ct: (isize, isize)) {
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
    if p.0 > *GRID_SIZE.lock().unwrap() || p.1 > *GRID_SIZE.lock().unwrap() {
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
    if p.0 > *GRID_SIZE.lock().unwrap() || p.1 > *GRID_SIZE.lock().unwrap() {
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
    // #[test]
    // fn testinput() {
    //     assert_eq!(run("./src/day18/testinput.txt", 6, 12), 22);
    // }
    #[test]
    fn input() {
        assert_eq!(run("./src/day18/input.txt", 70, 2929), 392);
    }
}
