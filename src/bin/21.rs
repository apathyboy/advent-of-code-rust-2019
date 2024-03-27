use advent_of_code::IntcodeComputer;

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<i128> {
    let mut computer = IntcodeComputer::new();

    computer.load_program_from_str(input);

    let input = r"NOT B J
    NOT C T
    OR T J
    AND D J
    NOT A T
    OR T J
    WALK
    ";

    for c in input.chars() {
        computer.set_input(c as i128);
    }

    computer.run();

    computer.get_next_output()
}

pub fn part_two(input: &str) -> Option<i128> {
    let mut computer = IntcodeComputer::new();

    computer.load_program_from_str(input);

    let input = r"NOT B J
NOT C T
OR T J
AND D J
AND H J
NOT A T
OR T J
RUN
";

    for c in input.chars() {
        computer.set_input(c as i128);
    }

    computer.run();

    computer.get_next_output()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_one() {
        assert_eq!(1, 1);
    }
}
