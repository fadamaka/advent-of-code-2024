use advent_of_code_2024::utils::util::read_lines;
use itertools::Itertools;

use super::b;

pub fn run(input_path: &str) -> String {
    let mut res_v: Vec<i32> = Vec::new();
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut instr_v: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines(input_path) {
        let mut count = 0;
        for line in lines.flatten() {
            if count == 0 {
                a = line.split(": ").collect_vec()[1].parse::<i32>().unwrap();
            }
            if count == 1 {
                b = line.split(": ").collect_vec()[1].parse::<i32>().unwrap();
            }
            if count == 2 {
                c = line.split(": ").collect_vec()[1].parse::<i32>().unwrap();
            }
            if count == 4 {
                instr_v = line
                    .split(": ")
                    .collect_vec()[1]
                    .split(",")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect_vec();
            }

            count += 1;
        }
        println!("A: {} B: {} C : {}", a, b, c);
        println!("instrs: {:?}", instr_v);

        let mut instr_p = 0;
        while instr_p < instr_v.len() {
            let opcode = instr_v[instr_p];
            let operand = instr_v[instr_p + 1];
            println!("opcode: {} operand: {}", opcode, operand);
            //adv
            if opcode == 0 {
                a = a / (2 as i32).pow(combo(a, b, c, operand) as u32);
            }
            //bdv
            if opcode == 6 {
                b = a / (2 as i32).pow(combo(a, b, c, operand) as u32);
            }
            //cdv
            if opcode == 7 {
                c = a / (2 as i32).pow(combo(a, b, c, operand) as u32);
            }
            //bxl
            if opcode == 1 {
                b = b ^ operand;
            }
            //bst
            if opcode == 2 {
                b = combo(a, b, c, operand) % 8;
            }
            //jnz
            if opcode == 3 && a != 0 {
                instr_p = operand as usize;
                continue;
            }
            //bxc
            if opcode == 4 {
                b = b ^ c;
            }
            //out
            if opcode == 5 {
                res_v.push(combo(a, b, c, operand) % 8);
            }
            instr_p += 2;
        }

        println!("A: {} B: {} C : {}", a, b, c);
        println!("instrs: {:?}", instr_v);
    }
    res_v
        .iter()
        .map(|x| x.to_string())
        .join(",")
}

fn combo(a: i32, b: i32, c: i32, operand: i32) -> i32 {
    if operand == 4 {
        return a;
    }
    if operand == 5 {
        return b;
    }
    if operand == 6 {
        return c;
    }
    operand
}

#[cfg(test)]
mod runs {
    use super::*;
    #[test]
    fn testinput() {
        assert_eq!(run("./src/day17/testinput.txt"), "4,6,3,5,6,3,5,2,1,0");
    }
    #[test]
    fn testinput2() {
        assert_eq!(run("./src/day17/testinput2.txt"), "");
    }
    #[test]
    fn input() {
        assert_eq!(run("./src/day17/input.txt"), "6,7,5,2,1,3,5,1,7");
    }
}
