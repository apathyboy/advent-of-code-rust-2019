use advent_of_code::IntcodeComputer;
advent_of_code::solution!(2);

const TARGET_OUTPUT: i128 = 19690720;

const NOUN_POSITION: usize = 1;
const NOUN_MAX: i128 = 99;
const DEFAULT_NOUN: i128 = 12;

const VERB_POSITION: usize = 2;
const VERB_MAX: i128 = 99;
const VERB_DEFAULT: i128 = 2;

const OUTPUT_REGISTER: usize = 0;
const OUTPUT_FACTOR: i128 = 100;

pub fn part_one(input: &str) -> Option<i128> {
    let mut computer = IntcodeComputer::default();

    computer.load_program_from_str(input);

    computer.set(NOUN_POSITION, DEFAULT_NOUN);
    computer.set(VERB_POSITION, VERB_DEFAULT);

    computer.run();

    computer.get(OUTPUT_REGISTER)
}

pub fn part_two(input: &str) -> Option<i128> {
    let mut computer = IntcodeComputer::default();

    for noun in 0..=NOUN_MAX {
        for verb in 0..=VERB_MAX {
            computer.load_program_from_str(input);

            computer.set(NOUN_POSITION, noun);
            computer.set(VERB_POSITION, verb);

            computer.run();

            if computer.get(OUTPUT_REGISTER)? == TARGET_OUTPUT {
                return Some(OUTPUT_FACTOR * noun + verb);
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
    #[case("1,9,10,3,2,3,11,0,99,30,40,50", Vec::from([3500,9,10,70,2,3,11,0,99,30,40,50]))]
    #[case("1,0,0,0,99", Vec::from([2,0,0,0,99]))]
    #[case("2,3,0,3,99", Vec::from([2,3,0,6,99]))]
    #[case("2,4,4,5,99,0", Vec::from([2,4,4,5,99,9801]))]
    #[case("1,1,1,4,99,5,6,0,99", Vec::from([30,1,1,4,2,5,6,0,99]))]
    fn test_run_program(#[case] input: &str, #[case] expected: Vec<i128>) {
        let mut computer = IntcodeComputer::new();

        computer.load_program_from_str(input);
        computer.run();

        let result = computer.memory_snapshot();
        assert_eq!(result, expected);
    }
}
