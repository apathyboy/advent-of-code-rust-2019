advent_of_code::solution!(16);

fn parse_input(input: &str) -> Vec<u32> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn phase(input: &[u32]) -> Vec<u32> {
    let mut output = Vec::new();

    for i in 0..input.len() {
        let mut sum = 0;
        for (j, &n) in input.iter().enumerate() {
            let pattern = match (j + 1) / (i + 1) % 4 {
                0 | 2 => 0,
                1 => 1,
                3 => -1,
                _ => unreachable!(),
            };
            sum += n as i32 * pattern;
        }
        output.push((sum.abs() % 10) as u32);
    }

    output
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut input = parse_input(input);

    for _ in 0..100 {
        input = phase(&input);
    }

    Some(input.iter().take(8).fold(0, |acc, &n| acc * 10 + n))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut input = parse_input(input);

    let offset = input.iter().take(7).fold(0, |acc, &n| acc * 10 + n) as usize;

    input = input
        .iter()
        .cycle()
        .take(input.len() * 10_000)
        .cloned()
        .collect();

    for _ in 0..100 {
        let mut sum = 0;
        for n in input.iter_mut().rev() {
            sum = (sum + *n as u32) % 10;
            *n = sum;
        }
    }

    Some(
        input
            .iter()
            .skip(offset)
            .take(8)
            .fold(0, |acc, &n| acc * 10 + n),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("80871224585914546619083218645595", 24176176)]
    #[case("19617804207202209144916044189917", 73745418)]
    #[case("69317163492948606335995924319873", 52432133)]
    fn test_part_one(#[case] input: &str, #[case] expected: u32) {
        let result = part_one(input);
        assert_eq!(result, Some(expected));
    }

    #[rstest]
    #[case("03036732577212944063491565474664", 84462026)]
    #[case("02935109699940807407585447034323", 78725270)]
    #[case("03081770884921959731165446850517", 53553731)]
    fn test_part_two(#[case] input: &str, #[case] expected: u32) {
        let result = part_two(input);
        assert_eq!(result, Some(expected));
    }
}
