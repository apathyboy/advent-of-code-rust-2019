advent_of_code::solution!(4);

fn is_valid_password(mut test: u32, part_of_larger_group: bool) -> bool {
    let mut prev_digit = 10; // No digit can be 10, ensuring the first comparison always fails if necessary.
    let mut has_double = false;
    let mut current_streak = 1; // Tracks the length of consecutive digit streaks

    while test > 0 {
        let digit = test % 10;
        test /= 10;

        // Check for decreasing sequence
        if digit > prev_digit {
            return false;
        }

        // Check for at least one double
        if digit == prev_digit {
            current_streak += 1;

            if part_of_larger_group && current_streak == 2 {
                has_double = true;
            }
        } else {
            if !part_of_larger_group && current_streak == 2 {
                has_double = true;
            }

            current_streak = 1; // Reset streak for a new digit
        }

        prev_digit = digit;
    }

    has_double || current_streak == 2 // Check has_double or if the last digits form a valid pair
}

fn parse(input: &str) -> Option<(u32, u32)> {
    let mut parts = input.trim().split('-');
    let min_password = parts.next()?.parse().ok()?;
    let max_password = parts.next()?.parse().ok()?;
    Some((min_password, max_password))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (min_password, max_password) = parse(input)?;

    (min_password..=max_password)
        .filter(|&test| is_valid_password(test, true))
        .count()
        .try_into()
        .ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    let (min_password, max_password) = parse(input)?;

    (min_password..=max_password)
        .filter(|&test| is_valid_password(test, false))
        .count()
        .try_into()
        .ok()
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
