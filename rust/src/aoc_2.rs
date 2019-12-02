pub fn aoc_2() {
    let mut program = [
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 13, 1, 19, 1, 19, 9, 23, 1, 5, 23, 27,
        1, 27, 9, 31, 1, 6, 31, 35, 2, 35, 9, 39, 1, 39, 6, 43, 2, 9, 43, 47, 1, 47, 6, 51, 2, 51,
        9, 55, 1, 5, 55, 59, 2, 59, 6, 63, 1, 9, 63, 67, 1, 67, 10, 71, 1, 71, 13, 75, 2, 13, 75,
        79, 1, 6, 79, 83, 2, 9, 83, 87, 1, 87, 6, 91, 2, 10, 91, 95, 2, 13, 95, 99, 1, 9, 99, 103,
        1, 5, 103, 107, 2, 9, 107, 111, 1, 111, 5, 115, 1, 115, 5, 119, 1, 10, 119, 123, 1, 13,
        123, 127, 1, 2, 127, 131, 1, 131, 13, 0, 99, 2, 14, 0, 0,
    ];
    program[1] = 12;
    program[2] = 2;
    let output = run(program.clone().to_vec());
    println!("Final output: {}", output);
    for i in 0..program.len() {
        for j in 0..program.len() {
            program[1] = i;
            program[2] = j;

            if 19690720 == run(program.clone().to_vec()) {
                println!("To get output 19690720 use inputs {}, {}", i, j);
                println!("Answer:  {}", i * 100 + j);
            }
        }
    }
}

fn run(mut program: Vec<usize>) -> usize {
    let mut pc = 0;
    while pc < program.len() {
        match program[pc] {
            1 => add(&mut program, &mut pc),
            2 => multiply(&mut program, &mut pc),
            99 => break,
            _ => break,
        }
    }
    program[0]
}

fn add(program: &mut [usize], pc: &mut usize) {
    let addr = program[*pc + 1] as usize;
    let add2 = program[*pc + 2] as usize;
    let add3 = program[*pc + 3] as usize;
    program[add3] = program[addr] + program[add2];
    *pc += 4;
}

fn multiply(program: &mut [usize], pc: &mut usize) {
    let addr = program[*pc + 1] as usize;
    let add2 = program[*pc + 2] as usize;
    let add3 = program[*pc + 3] as usize;
    program[add3] = program[addr] * program[add2];
    *pc += 4;
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn running_examples() {
        assert_eq!(run([1, 0, 0, 0, 99].to_vec()), 2);
        assert_eq!(run([2, 3, 0, 3, 99].to_vec()), 2);
        assert_eq!(run([2, 4, 4, 5, 99, 0].to_vec()), 2);
        assert_eq!(run([1, 1, 1, 4, 99, 5, 6, 0, 99].to_vec()), 30);
    }
}
