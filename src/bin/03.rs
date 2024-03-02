use std::collections::HashSet;

use glam::IVec2;

advent_of_code::solution!(3);

fn manhattan_distance(a: &IVec2) -> u32 {
    a.x.unsigned_abs() + a.y.unsigned_abs()
}

fn check_intersection(a: &[IVec2], b: &[IVec2]) -> Option<IVec2> {
    if a[0].x == a[1].x
        && b[0].y == b[1].y
        && (b[0].x.max(b[1].x) >= a[0].x && b[0].x.min(b[1].x) <= a[0].x)
        && (a[0].y.max(a[1].y) >= b[0].y && a[0].y.min(a[1].y) <= b[0].y)
    {
        Some(IVec2::new(a[0].x, b[0].y))
    } else if a[0].y == a[1].y
        && b[0].x == b[1].x
        && (a[0].x.max(a[1].x) >= b[0].x && a[0].x.min(a[1].x) <= b[0].x)
        && (b[0].y.max(b[1].y) >= a[0].y && b[0].y.min(b[1].y) <= a[0].y)
    {
        Some(IVec2::new(b[0].x, a[0].y))
    } else {
        None
    }
}

fn find_intersections(wire1: &[IVec2], wire2: &[IVec2]) -> HashSet<(u32, u32)> {
    let mut intersections = HashSet::new();

    let mut wire_1_distance = 0;

    for segment_1 in wire1.windows(2) {
        let mut wire_2_distance = 0;

        for segment_2 in wire2.windows(2) {
            if let Some(intersection) = check_intersection(segment_1, segment_2) {
                if intersection != IVec2::ZERO {
                    let direct_distance = manhattan_distance(&intersection);
                    let wire_distance = wire_1_distance
                        + (intersection - segment_1[0]).abs().x as u32
                        + (intersection - segment_1[0]).abs().y as u32
                        + wire_2_distance
                        + (intersection - segment_2[0]).abs().x as u32
                        + (intersection - segment_2[0]).abs().y as u32;

                    intersections.insert((direct_distance, wire_distance));
                }
            }

            wire_2_distance += (segment_2[0] - segment_2[1]).abs().x as u32
                + (segment_2[0] - segment_2[1]).abs().y as u32;
        }

        wire_1_distance += (segment_1[0] - segment_1[1]).abs().x as u32
            + (segment_1[0] - segment_1[1]).abs().y as u32;
    }

    intersections
}

fn parse_wire(line: &str) -> Option<Vec<IVec2>> {
    let mut prev_position = IVec2::ZERO;
    let mut wire = Vec::from([prev_position]);

    for segment in line.split(',') {
        let distance = &segment[1..].parse::<i32>().ok()?;
        let direction = &segment[0..1];

        let next_position = match direction {
            "U" => prev_position + IVec2::new(0, *distance),
            "D" => prev_position + IVec2::new(0, -*distance),
            "L" => prev_position + IVec2::new(-*distance, 0),
            "R" => prev_position + IVec2::new(*distance, 0),
            _ => panic!("Invalid direction"),
        };

        wire.push(next_position);

        prev_position = next_position;
    }

    Some(wire)
}

fn parse(input: &str) -> Option<(Vec<IVec2>, Vec<IVec2>)> {
    let lines = input.trim().lines().take(2).collect::<Vec<_>>();

    Some((parse_wire(lines[0])?, parse_wire(lines[1])?))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (wire1, wire2) = parse(input)?;

    find_intersections(&wire1, &wire2).iter().map(|i| i.0).min()
}

pub fn part_two(input: &str) -> Option<u32> {
    let (wire1, wire2) = parse(input)?;

    find_intersections(&wire1, &wire2).iter().map(|i| i.1).min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(159));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(610));
    }
}
