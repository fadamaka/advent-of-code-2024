use std::collections::HashSet;

use advent_of_code_2024::utils::util::read_lines;

pub fn run(input_path: &str) -> i32 {
    let mut result: i32 = 0;
    if let Ok(lines) = read_lines(input_path) {
        let mut matrix: Vec<Vec<_>> = Vec::new();
        for line in lines.flatten() {
            let chars: Vec<_> = line.chars().collect();
            matrix.push(chars);
        }
        let mut y: i32 = 0;
        let mut x: i32 = 0;
        for (i, row) in matrix.clone().iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if col == &'^' {
                    y = i as i32;
                    x = j as i32;
                    break;
                }
            }
            if x != 0 && y != 0 {
                break;
            }
        }

        println!("y: {} x: {}", y, x);

        for (i, row) in matrix.clone().iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if col == &'.' {
                    let mut matrix2 = matrix.clone();
                    let mut vec_c = matrix2.get(i).unwrap().clone();
                    vec_c[j] = '#';
                    matrix2[i] = vec_c;
                    println!("y: {}  x: {}", i, j);
                    result += brute_froce(y, x, matrix2);
                }
            }
        }
    }
    result
}

fn brute_froce(y_start: i32, x_start: i32, matrix: Vec<Vec<char>>) -> i32 {
    // for row in matrix.clone() {
    //     for col in row {
    //         print!("{}", col);
    //     }
    //     println!();
    // }
    // println!();
    let mut y = y_start;
    let mut x = x_start;
    let mut visited: HashSet<(i32, i32, i32, i32)> = HashSet::new();
    let directions: Vec<(i32, i32)> = Vec::from([
        (-1, 0),
        (0, 1),
        (1, 0),
        (0, -1),
    ]);
    let mut dir_i: usize = 0;
    let mut prev_len: usize = 0;
    while
        y >= 0 &&
        x >= 0 &&
        !matrix.get(y as usize).is_none() &&
        !matrix
            .get(y as usize)
            .unwrap()
            .get(x as usize)
            .is_none()
    {
        visited.insert((
            y.clone(),
            x.clone(),
            directions.get(dir_i).unwrap().0,
            directions.get(dir_i).unwrap().1,
        ));
        if prev_len == visited.len() {
            //println!("Loop found!");
            return 1;
        }
        prev_len = visited.len();
        let next_y = y + directions.get(dir_i).unwrap().0;
        let next_x = x + directions.get(dir_i).unwrap().1;
        if
            next_y >= 0 &&
            next_x >= 0 &&
            next_y < (matrix.len() as i32) &&
            next_x < (matrix.get(0).unwrap().len() as i32)
        {
            if
                matrix
                    .get(next_y as usize)
                    .unwrap()
                    .get(next_x as usize)
                    .unwrap() == &'#'
            {
                if dir_i < 3 {
                    dir_i += 1;
                } else {
                    dir_i = 0;
                }
                //new check

                let next_y = y + directions.get(dir_i).unwrap().0;
                let next_x = x + directions.get(dir_i).unwrap().1;
                if
                    next_y >= 0 &&
                    next_x >= 0 &&
                    next_y < (matrix.len() as i32) &&
                    next_x < (matrix.get(0).unwrap().len() as i32)
                {
                    if
                        matrix
                            .get(next_y as usize)
                            .unwrap()
                            .get(next_x as usize)
                            .unwrap() == &'#'
                    {
                        if dir_i < 3 {
                            dir_i += 1;
                        } else {
                            dir_i = 0;
                        }
                    }
                }
            }
        }
        y += directions.get(dir_i).unwrap().0;
        x += directions.get(dir_i).unwrap().1;
    }
    0
}

#[cfg(test)]
mod runs {
    use super::*;
    #[test]
    fn testinput() {
        assert_eq!(run("./src/day06/testinput.txt"), 6);
    }
    #[test]
    fn input() {
        assert_eq!(run("./src/day06/input.txt"), 0);
    }
}
