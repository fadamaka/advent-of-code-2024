use std::f32;

use advent_of_code_2024::utils::util::read_lines;

pub fn run(input_path: &str) -> i32 {
    let mut result: i32 = 0;
    //x*94+y*22 = 8400
    //x*34+y*67 = 5400
    //x*a+y*b = e
    //x*c+y*d = f
    solve(94 as f32, 22 as f32, 8400 as f32, 34 as f32, 67 as f32, 5400 as f32);
    if let Ok(lines) = read_lines(input_path) {
        for line in lines.flatten() {
            println!("{}", line);
        }
    }
    result
}

fn solve(a: f32, b: f32, e: f32, c: f32, d: f32, f: f32) -> (f32, f32) {
    let determinant = a * d - b * c;
    let x = (e * d - b * f) / determinant;
    let y = (a * f - e * c) / determinant;
    if x.fract() == 0.0 && y.fract() == 0.0 {
        println!("x:{:?} y:{:?}", x, y);
        return (x, y);
    }
    (0.0, 0.0)
}

#[cfg(test)]
mod runs {
    use super::*;
    #[test]
    fn testinput() {
        assert_eq!(run("./src/day13/testinput.txt"), 0);
    }
    #[test]
    fn input() {
        assert_eq!(run("./src/day13/input.txt"), 0);
    }
}
