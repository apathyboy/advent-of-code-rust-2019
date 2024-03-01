use advent_of_code::{parse_intcode_program, IntcodeComputer, IntcodeProgram};
advent_of_code::solution!(2);

fn run_program(program: IntcodeProgram) -> IntcodeProgram {
    let mut computer = IntcodeComputer::new();

    computer.load_program(program);
    computer.run();

    computer.read_program()
}

pub fn part_one(input: &str) -> Option<isize> {
    let mut program = parse_intcode_program(input)?;

    program[1] = 12;
    program[2] = 2;

    program = run_program(program);

    Some(program[0])
}

pub fn part_two(input: &str) -> Option<isize> {
    let mut program = parse_intcode_program(input)?;

    for noun in 0..=99 {
        for verb in 0..=99 {
            program[1] = noun;
            program[2] = verb;

            let output = run_program(program.clone());

            if output[0] == 19690720 {
                return Some(100 * noun + verb);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(IntcodeProgram::from([1,9,10,3,2,3,11,0,99,30,40,50]), IntcodeProgram::from([3500,9,10,70,2,3,11,0,99,30,40,50]))]
    #[case(IntcodeProgram::from([1,0,0,0,99]), IntcodeProgram::from([2,0,0,0,99]))]
    #[case(IntcodeProgram::from([2,3,0,3,99]), IntcodeProgram::from([2,3,0,6,99]))]
    #[case(IntcodeProgram::from([2,4,4,5,99,0]), IntcodeProgram::from([2,4,4,5,99,9801]))]
    #[case(IntcodeProgram::from([1,1,1,4,99,5,6,0,99]), IntcodeProgram::from([30,1,1,4,2,5,6,0,99]))]
    fn test_run_program(#[case] input: IntcodeProgram, #[case] expected: IntcodeProgram) {
        let result = run_program(input);
        assert_eq!(result, expected);
    }
}
