use glam::IVec2;
use std::collections::{HashMap, VecDeque};

use advent_of_code::IntcodeComputer;

advent_of_code::solution!(15);

const MOVE_NORTH: i128 = 1;
const MOVE_SOUTH: i128 = 2;
const MOVE_WEST: i128 = 3;
const MOVE_EAST: i128 = 4;

const STATUS_WALL: i128 = 0;
const STATUS_MOVED: i128 = 1;
const STATUS_OXYGEN: i128 = 2;

fn explore(computer: IntcodeComputer, map: &mut HashMap<IVec2, i128>, position: IVec2) {
    for direction in [MOVE_NORTH, MOVE_SOUTH, MOVE_WEST, MOVE_EAST].iter() {
        let new_position = match *direction {
            MOVE_NORTH => position + IVec2::new(0, 1),
            MOVE_SOUTH => position + IVec2::new(0, -1),
            MOVE_WEST => position + IVec2::new(-1, 0),
            MOVE_EAST => position + IVec2::new(1, 0),
            _ => panic!("Invalid direction"),
        };

        if map.contains_key(&new_position) {
            continue;
        }
        let mut computer = computer.clone();

        computer.set_input(*direction);

        let status = computer.run_to_next_output().unwrap();

        map.insert(new_position, status);

        if status != STATUS_WALL {
            explore(computer.clone(), map, new_position);
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut computer = IntcodeComputer::new();
    computer.load_program_from_str(input);

    let mut map = HashMap::new();

    let position = IVec2::new(0, 0);

    map.insert(position, 1);

    explore(computer, &mut map, position);

    map = map
        .iter()
        .filter(|(_, &status)| status != STATUS_WALL)
        .map(|(k, v)| (*k, *v))
        .collect();

    let oxygen_position = map
        .iter()
        .find(|(_, &status)| status == STATUS_OXYGEN)
        .unwrap()
        .0;

    let mut visited = HashMap::new();

    let mut queue = VecDeque::from([(IVec2::ZERO, 0)]);

    while let Some((position, distance)) = queue.pop_front() {
        if visited.contains_key(&position) {
            continue;
        }

        visited.insert(position, distance);

        if position == *oxygen_position {
            return Some(distance as u32);
        }

        for direction in [MOVE_NORTH, MOVE_SOUTH, MOVE_WEST, MOVE_EAST].iter() {
            let new_position = match *direction {
                MOVE_NORTH => position + IVec2::new(0, 1),
                MOVE_SOUTH => position + IVec2::new(0, -1),
                MOVE_WEST => position + IVec2::new(-1, 0),
                MOVE_EAST => position + IVec2::new(1, 0),
                _ => panic!("Invalid direction"),
            };

            if map.contains_key(&new_position) {
                queue.push_back((new_position, distance + 1));
            }
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut computer = IntcodeComputer::new();
    computer.load_program_from_str(input);

    let mut map = HashMap::new();

    let position = IVec2::new(0, 0);

    map.insert(position, 1);

    explore(computer, &mut map, position);

    let oxygen_position = map
        .iter()
        .find(|(_, &status)| status == STATUS_OXYGEN)
        .unwrap()
        .0;

    let mut to_process = vec![(*oxygen_position, 0)];

    let mut max_distance = 0;

    while let Some((position, distance)) = to_process.pop() {
        max_distance = max_distance.max(distance);

        for direction in [MOVE_NORTH, MOVE_SOUTH, MOVE_WEST, MOVE_EAST].iter() {
            let new_position = match *direction {
                MOVE_NORTH => position + IVec2::new(0, 1),
                MOVE_SOUTH => position + IVec2::new(0, -1),
                MOVE_WEST => position + IVec2::new(-1, 0),
                MOVE_EAST => position + IVec2::new(1, 0),
                _ => panic!("Invalid direction"),
            };

            if map.get(&new_position) == Some(&STATUS_MOVED) {
                to_process.push((new_position, distance + 1));
                map.insert(new_position, STATUS_OXYGEN);
            }
        }
    }

    Some(max_distance as u32)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_one() {
        assert_eq!(1, 1);
    }
}
