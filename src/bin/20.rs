use glam::IVec2;
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

    for (pos, t) in map.iter() {
        if let Tile::Portal(_, p) = t {
            println!("{:?} {:?}", pos, p);
        }
    }

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

    let mut visited = HashMap::new();
    let mut queue = vec![(*start, 0)];

    while let Some((pos, steps)) = queue.pop() {
        if visited.contains_key(&pos) {
            continue;
        }

        visited.insert(pos, steps);

        if pos == *end {
            return Some(steps);
        }

        let neighbors = [
            pos + IVec2::new(0, 1),
            pos + IVec2::new(0, -1),
            pos + IVec2::new(1, 0),
            pos + IVec2::new(-1, 0),
        ];

        for n in neighbors.into_iter() {
            if let Some(Tile::Empty) = map.get(&n) {
                queue.push((n, steps + 1));
            }
        }

        if let Some(Tile::Portal(_, _)) = map.get(&pos) {
            for (p, t) in map.iter() {
                if let Tile::Portal(_, _) = t {
                    if p != &pos {
                        queue.push((*p, steps + 1));
                    }
                }
            }
        }
    }

    visited.get(&end).copied()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
