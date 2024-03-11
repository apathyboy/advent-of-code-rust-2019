advent_of_code::solution!(8);

fn parse(input: &str) -> Vec<u32> {
    input
        .trim()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let data = parse(input);

    let mut zeros = usize::MAX;
    let mut checksum = 0;

    for layer in data.chunks(150) {
        let layer_zeros = layer.iter().filter(|&n| *n == 0).count();
        if layer_zeros < zeros {
            zeros = layer_zeros;

            let ones = layer.iter().filter(|&n| *n == 1).count();
            let twos = layer.iter().filter(|&n| *n == 2).count();

            checksum = ones * twos;
        }
    }

    Some(checksum)
}

pub fn part_two(input: &str) -> Option<String> {
    let data = parse(input);

    let layers = data.len() / 150;
    let mut image = Vec::new();

    for pixel in 0..150 {
        for i in 0..layers {
            let elem = i * 150 + pixel;
            if data[elem] != 2 {
                image.push(data[elem]);
                break;
            }
        }
    }

    let mut output = String::new();

    for row in image.chunks(25) {
        for p in row {
            if *p == 1 {
                output.push('#');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }

    Some(output)
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
