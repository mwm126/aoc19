pub fn aoc_7() {
    let program = [
        3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 42, 63, 76, 101, 114, 195, 276, 357, 438, 99999, 3,
        9, 101, 2, 9, 9, 102, 5, 9, 9, 1001, 9, 3, 9, 1002, 9, 5, 9, 4, 9, 99, 3, 9, 101, 4, 9, 9,
        102, 5, 9, 9, 1001, 9, 5, 9, 102, 2, 9, 9, 4, 9, 99, 3, 9, 1001, 9, 3, 9, 1002, 9, 5, 9, 4,
        9, 99, 3, 9, 1002, 9, 2, 9, 101, 5, 9, 9, 102, 3, 9, 9, 101, 2, 9, 9, 1002, 9, 3, 9, 4, 9,
        99, 3, 9, 101, 3, 9, 9, 102, 2, 9, 9, 4, 9, 99, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9,
        9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3,
        9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001,
        9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2,
        9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3,
        9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9,
        2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4,
        9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9,
        1001, 9, 1, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9,
        9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 99, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4,
        9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9,
        102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9,
        9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9,
        3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101,
        2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4,
        9, 3, 9, 101, 2, 9, 9, 4, 9, 99,
    ];
    let answer = max_signal(program.to_vec());
    println!(
        "The amplifier config for the maximum signal is:  {:?}",
        answer
    );
}

fn max_signal(program: Vec<i32>) -> (i32, i32, i32, i32, i32) {
    let mut max_output = 0;
    let mut max_config = (-1, -1, -1, -1, -1);
    for a in 1..=5 {
        for b in 1..=5 {
            for c in 1..=5 {
                for d in 1..=5 {
                    for e in 1..=5 {
                        let out_a = execute(program.clone().to_vec(), a, 0);
                        let out_b = execute(program.clone().to_vec(), b, out_a);
                        let out_c = execute(program.clone().to_vec(), c, out_b);
                        let out_d = execute(program.clone().to_vec(), d, out_c);
                        let out_e = execute(program.clone().to_vec(), e, out_d);
                        println!("checking... {} >? {}", out_e, max_output);
                        if out_e > max_output {
                            max_output = out_e;
                            max_config = (a, b, c, d, e);
                            println!("NEW MAXXXES!!! {:?} gives {}", max_config, max_output);
                        }
                    }
                }
            }
        }
    }
    max_config
}

fn execute(mut program: Vec<i32>, input: i32, input2: i32) -> i32 {
    let mut pc = 0;
    let mut output = 0;
    let mut inputs = vec![input2, input];
    while pc < program.len() {
        let opcode = program[pc] % 100;
        let pm = ((program[pc] / 100) % 10) == 0;
        let p2 = ((program[pc] / 1000) % 10) == 0;
        match opcode {
            1 => {
                let arg = get_arg(&program, pc + 1, pm);
                let arg2 = get_arg(&program, pc + 2, p2);
                let add3 = program[pc + 3] as usize;
                program[add3] = arg + arg2;
                pc += 4;
            }
            2 => {
                let arg = get_arg(&program, pc + 1, pm);
                let arg2 = get_arg(&program, pc + 2, p2);
                let add3 = program[pc + 3] as usize;
                program[add3] = arg * arg2;
                pc += 4;
            }
            3 => {
                let input_addr = program[pc + 1] as usize;
                program[input_addr] = inputs.pop().unwrap();
                println!("CHANGED point {} to value {}", input_addr, input);
                pc += 2;
            }
            4 => {
                output = get_arg(&program, pc + 1, pm);
                println!("OUTPUT: {}", output);
                pc += 2;
            }
            5 => {
                let arg = get_arg(&program, pc + 1, pm);
                let arg2 = get_arg(&program, pc + 2, p2);
                pc = if arg != 0 { arg2 as usize } else { pc + 3 };
            }
            6 => {
                let arg = get_arg(&program, pc + 1, pm);
                let arg2 = get_arg(&program, pc + 2, p2);
                pc = if arg == 0 { arg2 as usize } else { pc + 3 };
            }
            7 => {
                let arg = get_arg(&program, pc + 1, pm);
                let arg2 = get_arg(&program, pc + 2, p2);
                let add3 = program[pc + 3] as usize;
                program[add3] = if arg < arg2 { 1 } else { 0 };
                pc += 4;
            }
            8 => {
                let arg = get_arg(&program, pc + 1, pm);
                let arg2 = get_arg(&program, pc + 2, p2);
                let add3 = program[pc + 3] as usize;
                program[add3] = if arg == arg2 { 1 } else { 0 };
                pc += 4;
            }
            99 => break,
            opcode => panic!("Bad opcode: {}", opcode),
        }
    }
    output
}

fn get_arg(program: &[i32], pc: usize, pm: bool) -> i32 {
    let addr = program[pc];
    if pm {
        program[addr as usize]
    } else {
        addr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn immediate_mode() {
        assert_eq!(
            max_signal([3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0].to_vec()),
            (4, 3, 2, 1, 0)
        );
        assert_eq!(
            max_signal(
                [
                    3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23,
                    23, 4, 23, 99, 0, 0
                ]
                .to_vec()
            ),
            (0, 1, 2, 3, 4)
        );
        assert_eq!(
            max_signal(
                [
                    3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7,
                    33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
                ]
                .to_vec()
            ),
            (1, 0, 4, 3, 2)
        );
    }

    #[test]
    fn todo() {}
}
