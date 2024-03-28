use std::collections::HashMap;

use glam::IVec2;

advent_of_code::solution!(24);

fn parse_map(input: &str) -> HashMap<IVec2, bool> {
    let mut map = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = IVec2::new(x as i32, y as i32);
            map.insert(pos, c == '#');
        }
    }
    map
}

#[allow(dead_code)]
fn draw_map(map: &HashMap<IVec2, bool>) {
    let min_x = map.keys().map(|pos| pos.x).min().unwrap();
    let max_x = map.keys().map(|pos| pos.x).max().unwrap();
    let min_y = map.keys().map(|pos| pos.y).min().unwrap();
    let max_y = map.keys().map(|pos| pos.y).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let pos = IVec2::new(x, y);
            let c = if *map.get(&pos).unwrap_or(&false) {
                '#'
            } else {
                '.'
            };
            print!("{}", c);
        }
        println!();
    }
}

fn tick(map: &HashMap<IVec2, bool>) -> HashMap<IVec2, bool> {
    let mut new_map = HashMap::new();
    for (pos, is_bug) in map.iter() {
        let adjacent_bugs = [
            *pos + IVec2::Y,
            *pos + IVec2::NEG_Y,
            *pos + IVec2::X,
            *pos + IVec2::NEG_X,
        ]
        .iter()
        .filter(|&pos| *map.get(pos).unwrap_or(&false))
        .count();
        let new_is_bug = match (is_bug, adjacent_bugs) {
            (true, 1) => true,
            (true, _) => false,
            (false, 1..=2) => true,
            (false, _) => false,
        };
        new_map.insert(*pos, new_is_bug);
    }
    new_map
}

fn recursive_tick(recursive_map: &mut Vec<HashMap<IVec2, bool>>) {
    // check for bugs along the edges recursive levels of the map along with the 4 cardinal directions
    let mut new_recursive_map = recursive_map.to_vec();

    for (level, map) in recursive_map.iter().enumerate() {
        let min_x = map.keys().map(|pos| pos.x).min().unwrap();
        let max_x = map.keys().map(|pos| pos.x).max().unwrap();
        let min_y = map.keys().map(|pos| pos.y).min().unwrap();
        let max_y = map.keys().map(|pos| pos.y).max().unwrap();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if x == 2 && y == 2 {
                    continue;
                }

                let pos = IVec2::new(x, y);
                let adjacent_bugs = [
                    pos + IVec2::Y,
                    pos + IVec2::NEG_Y,
                    pos + IVec2::X,
                    pos + IVec2::NEG_X,
                ]
                .iter()
                .filter(|&pos| *map.get(pos).unwrap_or(&false))
                .count();
                let new_is_bug = match (map.get(&pos).unwrap_or(&false), adjacent_bugs) {
                    (true, 1) => true,
                    (true, _) => false,
                    (false, 1..=2) => true,
                    (false, _) => false,
                };
                new_recursive_map[level].insert(pos, new_is_bug);
            }
        }
    }
    *recursive_map = new_recursive_map;
}

fn biodiversity_rating(map: &HashMap<IVec2, bool>) -> u32 {
    let mut rating = 0;
    for (pos, is_bug) in map.iter() {
        if *is_bug {
            let index = pos.y * 5 + pos.x;
            rating |= 1 << index;
        }
    }
    rating
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = parse_map(input);

    let mut map_history = Vec::new();
    map_history.push(map.clone());

    loop {
        map = tick(&map);

        if map_history.contains(&map) {
            return Some(biodiversity_rating(&map));
        }

        map_history.push(map.clone());
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_map(input);

    // create an empty map for each level of recursion
    let mut empty_map = HashMap::new();
    for y in 0..5 {
        for x in 0..5 {
            if x == 2 && y == 2 {
                continue;
            }
            empty_map.insert(IVec2::new(x, y), false);
        }
    }

    let mut recursive_map = vec![empty_map; 201];

    recursive_map[100] = map.clone();

    for _ in 0..10 {
        recursive_tick(&mut recursive_map);
    }

    let bugs = recursive_map
        .iter()
        .map(|map| map.values().filter(|&&b| b).count())
        .sum::<usize>();

    Some(bugs as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2129920));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
