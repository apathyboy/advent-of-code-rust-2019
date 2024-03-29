use advent_of_code::IntcodeComputer;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<i128> {
    let mut computer = IntcodeComputer::default();

    computer.set_input(1);

    computer.load_program_from_str(input);
    computer.run();

    // return the last value in output
    computer.get_next_output()
}

pub fn part_two(input: &str) -> Option<i128> {
    let mut computer = IntcodeComputer::default();

    computer.set_input(5);

    computer.load_program_from_str(input);
    computer.run();

    // return the last value in output
    computer.get_next_output()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::position_mode_eq("3,9,8,9,10,9,4,9,99,-1,8", 8, 1)]
    #[case::position_mode_neq("3,9,8,9,10,9,4,9,99,-1,8", 32, 0)]
    #[case::position_mode_lt("3,9,7,9,10,9,4,9,99,-1,8", 2, 1)]
    #[case::position_mode_gt("3,9,7,9,10,9,4,9,99,-1,8", 32, 0)]
    #[case::immediate_mode_eq("3,3,1108,-1,8,3,4,3,99", 8, 1)]
    #[case::immediate_mode_neq("3,3,1108,-1,8,3,4,3,99", 32, 0)]
    #[case::immediate_mode_lt("3,3,1107,-1,8,3,4,3,99", 2, 1)]
    #[case::immediate_mode_gt("3,3,1107,-1,8,3,4,3,99", 32, 0)]
    #[case::position_mode_jump("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 0, 0)]
    #[case::position_mode_jump("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 32, 1)]
    #[case::immediate_mode_jump("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 0, 0)]
    #[case::immediate_mode_jump("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 32, 1)]
    fn test_simple_intcode(#[case] program: &str, #[case] input: i128, #[case] expected: i128) {
        let mut computer = IntcodeComputer::default();

        computer.set_input(input);

        computer.load_program_from_str(program);
        computer.run();

        // return the last value in output
        let result = computer.get_next_output();
        assert_eq!(result, Some(expected));
    }

    #[rstest]
    #[case(7, 999)]
    #[case(8, 1000)]
    #[case(9, 1001)]
    fn test_complex_intcode(#[case] input: i128, #[case] expected: i128) {
        let mut computer = IntcodeComputer::default();

        computer.set_input(input);

        computer.load_program_from_str(&advent_of_code::template::read_file("examples", DAY));
        computer.run();

        // return the last value in output
        let result = computer.get_next_output();
        assert_eq!(result, Some(expected));
    }
}
