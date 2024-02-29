advent_of_code::solution!(1);

fn calculate_required_fuel(mass: usize) -> usize {
    let fuel = (mass as isize / 3) - 2;

    if fuel > 0 {
        fuel as usize
    } else {
        0
    }
}

fn calculate_module_fuel(module_mass: usize) -> usize {
    let mut total_fuel = 0;

    let mut prev_fuel = calculate_required_fuel(module_mass);

    while prev_fuel > 0 {
        total_fuel += prev_fuel;
        prev_fuel = calculate_required_fuel(prev_fuel);
    }

    total_fuel
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .filter_map(|n| n.parse().ok())
            .map(calculate_required_fuel)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .filter_map(|n| n.parse().ok())
            .map(calculate_module_fuel)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(12, 2)]
    #[case(14, 2)]
    #[case(1969, 654)]
    #[case(100756, 33583)]
    fn test_calculate_required_fuel(#[case] input: usize, #[case] expected: usize) {
        let result = calculate_required_fuel(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(14, 2)]
    #[case(1969, 966)]
    #[case(100756, 50346)]
    fn test_calculate_module_fuel(#[case] input: usize, #[case] expected: usize) {
        let result = calculate_module_fuel(input);
        assert_eq!(result, expected);
    }
}
