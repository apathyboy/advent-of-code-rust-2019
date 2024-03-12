use advent_of_code::{parse_intcode_program, IntcodeComputer, IntcodeProgram};

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i128> {
    let program: IntcodeProgram = parse_intcode_program(input)?;
    let mut computer = IntcodeComputer::default();

    computer.set_input(1);

    computer.load_program(&program);
    computer.run();

    // return the last value in output
    computer.get_next_output()
}

pub fn part_two(input: &str) -> Option<i128> {
    let program: IntcodeProgram = parse_intcode_program(input)?;
    let mut computer = IntcodeComputer::default();

    computer.set_input(2);

    computer.load_program(&program);
    computer.run();

    // return the last value in output
    computer.get_next_output()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::relative_mode("104,1125899906842624,99", 1125899906842624)]
    #[case::large_num_mul("1102,34915192,34915192,7,4,7,99,0", 1219070632396864)]
    #[case::large_num_out("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99", 99)]
    fn test_intcode_changes(#[case] program: &str, #[case] expected: i128) {
        let program: IntcodeProgram = parse_intcode_program(program).unwrap();
        let mut computer = IntcodeComputer::default();

        computer.load_program(&program);
        computer.run();

        // return the last value in output
        let result = computer.get_next_output();
        assert_eq!(result, Some(expected));
    }
}
