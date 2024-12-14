use advent_of_code_2024::utils::util::read_lines;
use regex::Regex;

pub fn run(input_path: &str, x_c: i32, y_c: i32) -> i32 {
    let seconds = 100;
    let x_h = (x_c - 1) / 2;
    let y_h = (y_c - 1) / 2;
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    let re: Regex = Regex::new(r"-?\d+").unwrap();
    if let Ok(lines) = read_lines(input_path) {
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

            //println!("{} {}", x_r, y_r);
            if x_r < x_h && y_r < y_h {
                q1 += 1;
            }
            if x_r > x_h && y_r < y_h {
                q2 += 1;
            }
            if x_r > x_h && y_r > y_h {
                q3 += 1;
            }
            if x_r < x_h && y_r > y_h {
                q4 += 1;
            }
        }
    }
    //println!("{} {} {} {}", q1, q2, q3, q4);
    q1 * q2 * q3 * q4
}

#[cfg(test)]
mod runs {
    use super::*;
    #[test]
    fn testinput() {
        assert_eq!(run("./src/day14/testinput.txt", 11, 7), 12);
    }
    #[test]
    fn input() {
        assert_eq!(run("./src/day14/input.txt", 101, 103), 231019008);
    }
}
