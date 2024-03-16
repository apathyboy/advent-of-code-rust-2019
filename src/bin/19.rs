use advent_of_code::IntcodeComputer;

advent_of_code::solution!(19);

fn is_point_in_beam(x: i128, y: i128, mut computer: IntcodeComputer) -> bool {
    computer.set_input(x);
    computer.set_input(y);

    if let Some(output) = computer.run_until_output() {
        match output {
            0 => false,
            1 => true,
            _ => panic!("Invalid output from drone"),
        }
    } else {
        false
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut computer = IntcodeComputer::new();
    computer.load_program_from_str(input);

    let mut in_beam = 0;

    for y in 0..50 {
        for x in 0..50 {
            if is_point_in_beam(x, y, computer.clone()) {
                in_beam += 1;
            }
        }
    }

    Some(in_beam)
}

pub fn part_two(input: &str) -> Option<i128> {
    let mut computer = IntcodeComputer::new();
    computer.load_program_from_str(input);

    let mut x = 100;
    let mut y = 300;

    loop {
        while !is_point_in_beam(x, y, computer.clone()) {
            x += 1;
        }

        while is_point_in_beam(x + 99, y, computer.clone()) {
            if is_point_in_beam(x, y + 99, computer.clone())
                && is_point_in_beam(x + 99, y + 99, computer.clone())
            {
                return Some(x * 10_000 + y);
            }
            x += 1;
        }

        y += 1;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_one() {
        assert_eq!(1, 1);
    }
}
