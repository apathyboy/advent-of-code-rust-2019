use advent_of_code::IntcodeComputer;
use std::{collections::VecDeque, io};

advent_of_code::solution!(25);

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut computer = IntcodeComputer::new();
    computer.load_program_from_str(input);

    let mut output_buffer = String::new();

    let mut actions = VecDeque::from([
        "north",
        "east",
        "south",
        "take dehydrated water",
        "north",
        "west",
        "north",
        "east",
        "south",
        "take antenna",
        "west",
        "take hypercube",
        "east",
        "north",
        "west",
        "north",
        "east",
        "take candy cane",
        "west",
        "south",
        "south",
        "south",
        "west",
        "south",
        "west",
        "west",
    ]);

    loop {
        match computer.run_until_output() {
            Some(10) => {
                println!("{}", output_buffer);

                if output_buffer.contains("Command?") {
                    let input = if let Some(action) = actions.pop_front() {
                        action.to_owned()
                    } else {
                        read_input()
                    };

                    computer.add_input_str(&input);
                }

                output_buffer.clear();
            }
            Some(c) => {
                output_buffer.push(c as u8 as char);
            }
            None => break,
        }
    }

    println!();

    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_one() {
        assert_eq!(1, 1);
    }
}
