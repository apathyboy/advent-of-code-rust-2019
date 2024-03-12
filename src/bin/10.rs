use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
};

use glam::Vec2;

advent_of_code::solution!(10);

struct Map {
    asteroids: Vec<Vec2>,
    width: usize,
    height: usize,
}

impl Map {
    fn new() -> Self {
        Self {
            asteroids: Vec::new(),
            width: 0,
            height: 0,
        }
    }
}

#[derive(Debug, Clone)]
struct Vec2Key(Vec2);

impl PartialEq for Vec2Key {
    fn eq(&self, other: &Self) -> bool {
        // Implement your equality logic here, possibly with some epsilon comparison for floating-point values
        (self.0 - other.0).length_squared() < 1e-6
    }
}

impl Eq for Vec2Key {}

impl Hash for Vec2Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Implement your hashing logic here. For example, by discretizing the space
        let discretized_x = (self.0.x * 1000.0).round() as i64;
        let discretized_y = (self.0.y * 1000.0).round() as i64;
        discretized_x.hash(state);
        discretized_y.hash(state);
    }
}

fn parse_map(input: &str) -> Option<Map> {
    let mut map = Map::new();

    for (y, line) in input.lines().enumerate() {
        if y > map.height {
            map.height = y;
        }

        for (x, c) in line.chars().enumerate() {
            if x > map.width {
                map.width = x;
            }

            if c == '#' {
                map.asteroids.push(Vec2::new(x as f32, y as f32));
            }
        }
    }

    Some(map)
}

fn find_best_asteroid(map: &Map) -> Option<(Vec2, usize)> {
    let mut max_asteroids_detected = 0;
    let mut best_asteroid = None;

    for asteroid in map.asteroids.iter() {
        let mut visible = HashSet::new();

        for other in map.asteroids.iter() {
            if asteroid == other {
                continue;
            }
            let unit_direction = (*other - *asteroid).normalize();

            visible.insert(Vec2Key(unit_direction));
        }

        if visible.len() > max_asteroids_detected {
            max_asteroids_detected = visible.len();
            best_asteroid = Some(*asteroid);
        }
    }

    Some((best_asteroid?, max_asteroids_detected))
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = parse_map(input)?;

    let (_, max_asteroids_detected) = find_best_asteroid(&map)?;

    Some(max_asteroids_detected)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_map(input)?;

    let (best_asteroid, _) = find_best_asteroid(&map)?;

    let mut angles = HashMap::new();

    for other in &map.asteroids {
        if best_asteroid == *other {
            continue;
        }

        let angle = (*other - best_asteroid).normalize();
        let angle_key = Vec2Key(angle);
        let distance = (*other - best_asteroid).length_squared();

        let entry = angles
            .entry(angle_key)
            .or_insert((Vec2::ZERO, std::f32::MAX));
        if distance < entry.1 {
            *entry = (*other, distance);
        }
    }

    let mut sorted_angles: Vec<_> = angles.iter().collect();

    sorted_angles.sort_by(|a, b| {
        let a_angle = a.0 .0;
        let b_angle = b.0 .0;

        let a_angle = if a_angle.y < 0.0 {
            2.0 * std::f32::consts::PI - a_angle.angle_between(Vec2::Y)
        } else {
            a_angle.angle_between(Vec2::Y)
        };

        let b_angle = if b_angle.y < 0.0 {
            2.0 * std::f32::consts::PI - b_angle.angle_between(Vec2::Y)
        } else {
            b_angle.angle_between(Vec2::Y)
        };

        a_angle.partial_cmp(&b_angle).unwrap()
    });

    let mut count = 0;

    let mut last_angle = Vec2Key(Vec2::ZERO);

    for (angle, (pos, _)) in sorted_angles {
        if *angle != last_angle {
            count += 1;
            if count == 200 {
                return Some((pos.x * 100.0 + pos.y) as u32);
            }
        }

        let Vec2Key(angle) = *angle;

        last_angle = Vec2Key(angle);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(210));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(802));
    }
}
