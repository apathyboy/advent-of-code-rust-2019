use advent_of_code::{parse_intcode_program, IntcodeComputer, IntcodeProgram};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let program: IntcodeProgram = parse_intcode_program(input)?;
    let mut computer = IntcodeComputer::default();

    let input_source = Box::new(|| 1);

    computer.init_input_source(input_source);

    computer.load_program(&program);
    computer.run();

    // return the last value in output
    computer.get_output().last().copied().map(|val| val as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let program: IntcodeProgram = parse_intcode_program(input)?;
    let mut computer = IntcodeComputer::default();

    let input_source = Box::new(|| 5);

    computer.init_input_source(input_source);

    computer.load_program(&program);
    computer.run();

    // return the last value in output
    computer.get_output().last().copied().map(|val| val as u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(7, 999)]
    #[case(8, 1000)]
    #[case(9, 1001)]
    fn test_part_two(#[case] input: i64, #[case] expected: u32) {
        let program: IntcodeProgram =
            parse_intcode_program(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        let mut computer = IntcodeComputer::default();

        let input_source = Box::new(move || input);

        computer.init_input_source(input_source);

        computer.load_program(&program);
        computer.run();

        // return the last value in output
        let result = computer.get_output().last().copied().map(|val| val as u32);
        assert_eq!(result, Some(expected));
    }

    #[rstest]
    #[case(0, 0)]
    #[case(32, 1)]
    fn test_part_two2(#[case] input: i64, #[case] expected: u32) {
        let program: IntcodeProgram =
            parse_intcode_program("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9").unwrap();
        let mut computer = IntcodeComputer::default();

        let input_source = Box::new(move || input);

        computer.init_input_source(input_source);

        computer.load_program(&program);
        computer.run();

        // return the last value in output
        let result = computer.get_output().last().copied().map(|val| val as u32);
        assert_eq!(result, Some(expected));
    }

    #[rstest]
    #[case(0, 0)]
    #[case(32, 1)]
    fn test_part_two3(#[case] input: i64, #[case] expected: u32) {
        let program: IntcodeProgram =
            parse_intcode_program("3,3,1105,-1,9,1101,0,0,12,4,12,99,1").unwrap();
        let mut computer = IntcodeComputer::default();

        let input_source = Box::new(move || input);

        computer.init_input_source(input_source);

        computer.load_program(&program);
        computer.run();

        // return the last value in output
        let result = computer.get_output().last().copied().map(|val| val as u32);
        assert_eq!(result, Some(expected));
    }

    #[rstest]
    #[case(8, 1)]
    #[case(32, 0)]
    fn test_part_two4(#[case] input: i64, #[case] expected: u32) {
        let program: IntcodeProgram = parse_intcode_program("3,9,8,9,10,9,4,9,99,-1,8").unwrap();
        let mut computer = IntcodeComputer::default();

        let input_source = Box::new(move || input);

        computer.init_input_source(input_source);

        computer.load_program(&program);
        computer.run();

        // return the last value in output
        let result = computer.get_output().last().copied().map(|val| val as u32);
        assert_eq!(result, Some(expected));
    }

    #[rstest]
    #[case(2, 1)]
    #[case(32, 0)]
    fn test_part_two5(#[case] input: i64, #[case] expected: u32) {
        let program: IntcodeProgram = parse_intcode_program("3,9,7,9,10,9,4,9,99,-1,8").unwrap();
        let mut computer = IntcodeComputer::default();

        let input_source = Box::new(move || input);

        computer.init_input_source(input_source);

        computer.load_program(&program);
        computer.run();

        // return the last value in output
        let result = computer.get_output().last().copied().map(|val| val as u32);
        assert_eq!(result, Some(expected));
    }

    #[rstest]
    #[case(8, 1)]
    #[case(32, 0)]
    fn test_part_two6(#[case] input: i64, #[case] expected: u32) {
        let program: IntcodeProgram = parse_intcode_program("3,3,1108,-1,8,3,4,3,99").unwrap();
        let mut computer = IntcodeComputer::default();

        let input_source = Box::new(move || input);

        computer.init_input_source(input_source);

        computer.load_program(&program);
        computer.run();

        // return the last value in output
        let result = computer.get_output().last().copied().map(|val| val as u32);
        assert_eq!(result, Some(expected));
    }

    #[rstest]
    #[case(2, 1)]
    #[case(32, 0)]
    fn test_part_two7(#[case] input: i64, #[case] expected: u32) {
        let program: IntcodeProgram = parse_intcode_program("3,3,1107,-1,8,3,4,3,99").unwrap();
        let mut computer = IntcodeComputer::default();

        let input_source = Box::new(move || input);

        computer.init_input_source(input_source);

        computer.load_program(&program);
        computer.run();

        // return the last value in output
        let result = computer.get_output().last().copied().map(|val| val as u32);
        assert_eq!(result, Some(expected));
    }
}
