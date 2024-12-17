use advent_of_code_2024::utils::util::read_lines;
use itertools::Itertools;

pub fn run(input_path: &str) -> String {
    let mut res_v: Vec<i64> = Vec::new();

    let mut res_v2: Vec<i64> = Vec::new();
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut instr_v: Vec<i64> = Vec::new();
    if let Ok(lines) = read_lines(input_path) {
        let mut count = 0;
        for line in lines.flatten() {
            if count == 0 {
                a = line.split(": ").collect_vec()[1].parse::<i64>().unwrap();
            }
            if count == 1 {
                b = line.split(": ").collect_vec()[1].parse::<i64>().unwrap();
            }
            if count == 2 {
                c = line.split(": ").collect_vec()[1].parse::<i64>().unwrap();
            }
            if count == 4 {
                instr_v = line
                    .split(": ")
                    .collect_vec()[1]
                    .split(",")
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect_vec();
            }

            count += 1;
        }
        println!("A: {} B: {} C : {}", a, b, c);
        println!("instrs: {:?}", instr_v);
        //81000000000036
        let test = 234142032285293;
        a = test;
        let mut instr_p = 0;
        while instr_p < instr_v.len() {
            let opcode = instr_v[instr_p];
            let operand = instr_v[instr_p + 1];
            // println!("opcode: {} operand: {}", opcode, operand);
            // println!("a: {}", a);
            //adv
            if opcode == 0 {
                a = a / (2 as i64).pow(combo(a, b, c, operand) as u32);
            }
            //bdv
            if opcode == 6 {
                b = a / (2 as i64).pow(combo(a, b, c, operand) as u32);
            }
            //cdv
            if opcode == 7 {
                c = a / (2 as i64).pow(combo(a, b, c, operand) as u32);
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
                println!("a: {}", a);
                println!(
                    "b: {} before mod: {} output: {} ",
                    b,
                    combo(a, b, c, operand),
                    combo(a, b, c, operand) % 8
                );
                res_v.push(combo(a, b, c, operand) % 8);
            }
            instr_p += 2;
        }
        a = test;
        b = 0;
        c = 0;
        println!("manual");
        while a != 0 {
            println!("a: {}", a);
            println!("output: {}", calc_output(a));
            res_v2.push(calc_output(a));
            a = a / 8;
        }
        instr_v.reverse();
        let mut result = 0;
        println!("reverse");
        for i in 0..instr_v.len() {
            let mut found = false;
            let mut count_f = 0;
            for j in 0..7 {
                if instr_v[i] == calc_output(result + j) {
                    if !found {
                        result += j;
                    }
                    count_f += 1;
                    found = true;
                }
            }
            if !found {
                println!("error at {} a: {}", i, result);
                for j in 0..70 {
                    if instr_v[i] == calc_output(result + j) {
                        println!("found at {}", j);
                    }
                }
                break;
            }
            println!("count: {}", count_f);
            println!("a: {}", result);
            println!("output: {}", instr_v[i]);
            result *= 8;
        }

        println!("A: {} B: {} C : {}", a, b, c);
        //println!("instrs: {:?}", instr_v);
    }
    let res = res_v
        .iter()
        .map(|x| x.to_string())
        .join(",");
    println!(
        "{}",
        res_v2
            .iter()
            .map(|x| x.to_string())
            .join(",")
    );
    println!("{}", res);
    solve(&instr_v, 0, 0);
    res_v2
        .iter()
        .map(|x| x.to_string())
        .join(",")
}

fn solve(res_v: &Vec<i64>, i: usize, a: i64) {
    //println!("searching match for {}", res_v[i]);
    if i == 16 {
        let mut a_i = a / 8;
        let mut res_v2: Vec<i64> = Vec::new();
        while a_i != 0 {
            res_v2.push(calc_output(a_i));
            a_i = a_i / 8;
        }
        res_v2.reverse();
        let matching = res_v
            .iter()
            .zip(res_v2.iter())
            .filter(|&(a, b)| a == b)
            .count();
        if matching == res_v.len() && matching == res_v2.len() {
            println!("result: {}", a / 8);
        }
        return;
    }
    for j in 0..8 {
        if res_v[i] == calc_output(a + j) {
            if i < res_v.len() {
                //println!("found a: {} for output: {}", a + j, res_v[i]);
                solve(res_v, i + 1, (a + j) * 8);
            } else {
                println!("result: {}", a + j);
            }
        }
    }
}

fn calc_output(a: i64) -> i64 {
    let mut b = a % 8 ^ 3;
    let c = a / (2 as i64).pow(b as u32);
    b = b ^ 5;
    b = b ^ c;
    b % 8
}

fn combo(a: i64, b: i64, c: i64, operand: i64) -> i64 {
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
    //#[test]
    // fn testinput() {
    //     assert_eq!(run("./src/day17/testinput.txt"), "4,6,3,5,6,3,5,2,1,0");
    // }
    // #[test]
    // fn testinput2() {
    //     assert_eq!(run("./src/day17/testinput2.txt"), "");
    // }
    #[test]
    fn input() {
        assert_eq!(run("./src/day17/input.txt"), "2,4,1,3,7,5,1,5,0,3,4,1,5,5,3,0");
    }
}
