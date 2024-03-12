use advent_of_code::{parse_intcode_program, IntcodeComputer, IntcodeProgram};
use itertools::Itertools;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<i128> {
    let program: IntcodeProgram = parse_intcode_program(input)?;

    let mut max_thruster_signal = 0;

    let phase_settings = [0, 1, 2, 3, 4];
    for p in phase_settings.iter().permutations(phase_settings.len()) {
        let mut input_signal = 0;

        for amplifier in p {
            let mut computer = IntcodeComputer::default();

            computer.set_input(*amplifier);
            computer.set_input(input_signal);

            computer.load_program(&program);
            computer.run();

            // return the last value in output
            input_signal = computer.get_next_output()?;
        }

        if input_signal > max_thruster_signal {
            max_thruster_signal = input_signal;
        }
    }

    Some(max_thruster_signal)
}

pub fn part_two(input: &str) -> Option<i128> {
    let program: IntcodeProgram = parse_intcode_program(input)?;

    let mut max_thruster_signal = 0;

    let phase_settings = [5, 6, 7, 8, 9];
    for p in phase_settings.iter().permutations(phase_settings.len()) {
        let mut input_signal = 0;
        let mut amplifiers = Vec::new();

        for amplifier in p {
            let mut computer = IntcodeComputer::default();

            computer.load_program(&program);

            computer.set_input(*amplifier);

            amplifiers.push(computer);
        }

        loop {
            for amplifier in amplifiers.iter_mut() {
                amplifier.set_input(input_signal);

                while amplifier.is_running() && amplifier.get_output().is_empty() {
                    amplifier.tick();
                }

                if let Some(output) = amplifier.get_next_output() {
                    input_signal = output;
                }
            }

            if !amplifiers[0].is_running() {
                break;
            }
        }

        if input_signal > max_thruster_signal {
            max_thruster_signal = input_signal;
        }
    }

    Some(max_thruster_signal)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43210));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(139629729));
    }
}
