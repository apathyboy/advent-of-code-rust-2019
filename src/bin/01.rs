use std::iter::successors;

advent_of_code::solution!(1);

fn calculate_required_fuel(mass: usize) -> usize {
    ((mass / 3).saturating_sub(2)).max(0)
}

fn calculate_module_fuel(module_mass: usize) -> usize {
    successors(Some(calculate_required_fuel(module_mass)), |&mass| {
        let fuel = calculate_required_fuel(mass);
        if fuel == 0 {
            None
        } else {
            Some(fuel)
        }
    })
    .sum()
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
