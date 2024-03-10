use advent_of_code::{parse_intcode_program, IntcodeComputer, IntcodeProgram};
use std::cell::RefCell;
use std::rc::Rc;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let output = Rc::new(RefCell::new(Vec::new()));

    let program: IntcodeProgram = parse_intcode_program(input)?;
    let mut computer = IntcodeComputer::default();

    let input_source = Box::new(|| 1);

    // Clone the Rc to have another owner of the output. The original `output` is still usable after the move.
    let output_clone = Rc::clone(&output);
    let output_sink = Box::new(move |val: i64| {
        output_clone.borrow_mut().push(val);
    });

    computer.init_input_source(input_source);
    computer.init_output_sink(output_sink);

    computer.load_program(&program);
    computer.run();

    // return the last value in output
    let x = output.borrow().last().copied().map(|val| val as u32);
    x
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
