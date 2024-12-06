use std::collections::HashSet;

use advent_of_code_2024::utils::util::read_lines;

pub fn run(input_path: &str) -> i32 {
    let mut result: i32 = 0;
    if let Ok(lines) = read_lines(input_path) {
        let mut matrix: Vec<Vec<_>> = Vec::new();
        for line in lines.flatten() {
            let chars: Vec<_> = line.chars().collect();
            matrix.push(chars);
            println!("{}", line);
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
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let directions: Vec<(i32, i32)> = Vec::from([
            (-1, 0),
            (0, 1),
            (1, 0),
            (0, -1),
        ]);
        let mut dir_i: usize = 0;
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
            visited.insert((y.clone(), x.clone()));
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
            y += directions.get(dir_i).unwrap().0;
            x += directions.get(dir_i).unwrap().1;
        }
        result = visited.len() as i32;
    }
    result
}

#[cfg(test)]
mod runs {
    use super::*;
    #[test]
    fn testinput() {
        assert_eq!(run("./src/day06/testinput.txt"), 41);
    }
    #[test]
    fn input() {
        assert_eq!(run("./src/day06/input.txt"), 4602);
    }
}
