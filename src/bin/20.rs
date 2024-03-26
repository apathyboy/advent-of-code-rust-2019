use glam::IVec2;
use pathfinding::prelude::bfs;
use std::{collections::HashMap, hash::Hash};

advent_of_code::solution!(20);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Portal(IVec2, String),
}

fn parse_map_with_portals(input: &str) -> HashMap<IVec2, Tile> {
    let mut map = HashMap::new();

    let lines = input.lines().collect::<Vec<_>>();

    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            let c = lines[y].chars().nth(x).unwrap();
            let pos = IVec2::new(x as i32, y as i32);

            if c == '.' {
                map.entry(pos).or_insert(Tile::Empty);
            } else if c.is_ascii_uppercase() {
                let mut portal = String::new();

                if x + 1 < lines[y].len()
                    && lines[y].chars().nth(x + 1).unwrap().is_ascii_uppercase()
                {
                    portal.push(c);
                    portal.push(lines[y].chars().nth(x + 1).unwrap());
                    if x + 2 < lines[y].len() && lines[y].chars().nth(x + 2).unwrap() == '.' {
                        map.insert(pos + IVec2::new(2, 0), Tile::Portal(pos, portal.clone()));
                    } else {
                        map.insert(pos - IVec2::new(1, 0), Tile::Portal(pos, portal.clone()));
                    }
                } else if y + 1 < lines.len()
                    && lines[y + 1].chars().nth(x).unwrap().is_ascii_uppercase()
                {
                    portal.push(c);
                    portal.push(lines[y + 1].chars().nth(x).unwrap());
                    if y + 2 < lines.len() && lines[y + 2].chars().nth(x).unwrap() == '.' {
                        map.insert(pos + IVec2::new(0, 2), Tile::Portal(pos, portal.clone()));
                    } else {
                        map.insert(pos - IVec2::new(0, 1), Tile::Portal(pos, portal.clone()));
                    }
                }
            }
        }
    }

    map
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_map_with_portals(input);

    let start = map
        .iter()
        .find(|(_, t)| {
            if let Tile::Portal(_, p) = t {
                p == "AA"
            } else {
                false
            }
        })
        .unwrap()
        .0;

    let end = map
        .iter()
        .find(|(_, t)| {
            if let Tile::Portal(_, p) = t {
                p == "ZZ"
            } else {
                false
            }
        })
        .unwrap()
        .0;

    let result = bfs(
        start,
        |&pos| {
            let mut neighbors = Vec::new();

            for dir in &[
                IVec2::new(0, 1),
                IVec2::new(0, -1),
                IVec2::new(1, 0),
                IVec2::new(-1, 0),
            ] {
                let new_pos = pos + *dir;
                if map.contains_key(&new_pos) {
                    neighbors.push(new_pos);
                }
            }

            if let Some(Tile::Portal(_, side1)) = map.get(&pos) {
                if let Some((p, _)) = map.iter().find(|(p, t)| {
                    if let Tile::Portal(_, side2) = t {
                        **p != pos && side1 == side2
                    } else {
                        false
                    }
                }) {
                    neighbors.push(*p);
                }
            }

            neighbors
        },
        |&pos| pos == *end,
    );

    result?.len().checked_sub(1).map(|x| x as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_map_with_portals(input);

    let start = map
        .iter()
        .find(|(_, t)| {
            if let Tile::Portal(_, p) = t {
                p == "AA"
            } else {
                false
            }
        })
        .unwrap()
        .0;

    let end = map
        .iter()
        .find(|(_, t)| {
            if let Tile::Portal(_, p) = t {
                p == "ZZ"
            } else {
                false
            }
        })
        .unwrap()
        .0;

    let min = map.keys().fold(IVec2::new(i32::MAX, i32::MAX), |acc, p| {
        IVec2::new(acc.x.min(p.x), acc.y.min(p.y))
    });
    let max = map.keys().fold(IVec2::new(i32::MIN, i32::MIN), |acc, p| {
        IVec2::new(acc.x.max(p.x), acc.y.max(p.y))
    });

    let is_outer_teleport = move |pos: IVec2| -> bool {
        pos.x == min.x || pos.y == min.y || pos.x == max.x || pos.y == max.y
    };

    let result = bfs(
        &(*start, 0),
        |&(pos, level)| {
            let mut neighbors = Vec::new();

            for dir in &[
                IVec2::new(0, 1),
                IVec2::new(0, -1),
                IVec2::new(1, 0),
                IVec2::new(-1, 0),
            ] {
                let new_pos = pos + *dir;
                if map.contains_key(&new_pos) {
                    neighbors.push((new_pos, level));
                }
            }

            if let Some(Tile::Portal(_, side1)) = map.get(&pos) {
                if let Some((p, _)) = map.iter().find(|(p, t)| {
                    if let Tile::Portal(_, side2) = t {
                        **p != pos && side1 == side2
                    } else {
                        false
                    }
                }) {
                    if is_outer_teleport(pos) {
                        if level > 0 {
                            neighbors.push((*p, level - 1));
                        }
                    } else {
                        neighbors.push((*p, level + 1));
                    }
                }
            }

            neighbors
        },
        |&(pos, level)| pos == *end && level == 0,
    );

    result?.len().checked_sub(1).map(|x| x as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(23));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(396));
    }
}
