use std::{
    cmp::Ordering,
    collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};

use glam::IVec2;

advent_of_code::solution!(18);

fn neighbors(pos: IVec2) -> Vec<IVec2> {
    vec![
        pos + IVec2::new(0, 1),
        pos + IVec2::new(0, -1),
        pos + IVec2::new(1, 0),
        pos + IVec2::new(-1, 0),
    ]
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Wall,
    Empty,
    Node(char),
}

#[derive(PartialEq, Eq)]
struct DijkstraState {
    cost: usize,
    current: char,
}

impl Ord for DijkstraState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.current.cmp(&other.current))
    }
}

impl PartialOrd for DijkstraState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_grid(input: &str) -> HashMap<IVec2, Tile> {
    let mut grid = HashMap::new();
    for (height, line) in input.trim().lines().enumerate() {
        for (width, c) in line.chars().enumerate() {
            let tile = match c {
                '#' => Tile::Wall,
                '.' => Tile::Empty,
                _ => Tile::Node(c),
            };
            grid.insert(IVec2::new(width as i32, height as i32), tile);
        }
    }
    grid
}

fn graph(grid: &HashMap<IVec2, Tile>) -> HashMap<char, HashMap<char, usize>> {
    let mut graph = HashMap::new();
    for (coord, tile) in grid.iter() {
        if let Tile::Node(c) = tile {
            let pos_edges = reachable_from(grid, *coord);
            graph.insert(*c, pos_edges);
        }
    }

    graph
}

fn reachable_from(grid: &HashMap<IVec2, Tile>, coord: IVec2) -> HashMap<char, usize> {
    let mut visited = HashSet::new();
    let mut result = HashMap::new();

    let mut queue = VecDeque::new();
    queue.push_back((coord, 0));

    visited.insert(coord);
    while let Some((current, steps)) = queue.pop_front() {
        for neighbor in neighbors(current) {
            if let Some(tile) = grid.get(&neighbor) {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    match tile {
                        Tile::Empty => {
                            queue.push_back((neighbor, steps + 1));
                        }
                        Tile::Node(c) => {
                            result.insert(*c, steps + 1);
                        }
                        Tile::Wall => {}
                    }
                }
            }
        }
    }
    result
}

#[derive(PartialEq, Eq)]
struct State {
    steps: usize,
    node: char,
    keys: BTreeSet<char>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .steps
            .cmp(&self.steps)
            .then(self.keys.len().cmp(&other.keys.len()))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn search(graph: HashMap<char, HashMap<char, usize>>, start: char) -> usize {
    let mut priority_queue = BinaryHeap::new();
    let key_count = graph.iter().filter(|(k, _)| k.is_lowercase()).count();

    // keep track of best cost at (robot_position, keys collected)
    let mut distances: HashMap<(char, BTreeSet<char>), usize> = HashMap::new();
    distances.insert((start, BTreeSet::new()), 0);

    let start = State {
        steps: 0,
        node: start,
        keys: BTreeSet::new(),
    };

    priority_queue.push(start);

    // search keys cache, avoid recomputing search keys for the same (position, keys collected)
    let mut cache = HashMap::new();

    while let Some(current) = priority_queue.pop() {
        if current.keys.len() == key_count {
            return current.steps;
        }

        if let Some(&best_steps) = distances.get(&(current.node, current.keys.clone())) {
            if current.steps > best_steps {
                continue;
            }
        }

        let cache_key = (current.node, current.keys.clone());

        let cached_entry = cache
            .entry(cache_key)
            .or_insert_with(|| search_keys(&graph, &current.keys, current.node));

        for &(next_node, cost) in cached_entry.iter() {
            let mut next_keys = current.keys.clone();
            next_keys.insert(next_node);
            let next_steps = current.steps + cost;

            let distances_entry = distances
                .entry((next_node, next_keys.clone()))
                .or_insert(usize::max_value());

            if next_steps < *distances_entry {
                *distances_entry = next_steps;

                let next_state = State {
                    steps: current.steps + cost,
                    node: next_node,
                    keys: next_keys,
                };

                priority_queue.push(next_state);
            }
        }
    }
    // no path found
    usize::max_value()
}

fn search_keys(
    graph: &HashMap<char, HashMap<char, usize>>,
    keys: &BTreeSet<char>,
    start: char,
) -> Vec<(char, usize)> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist = HashMap::new();
    for &key in graph.keys() {
        dist.insert(key, usize::max_value());
    }

    let mut heap = BinaryHeap::new();

    *dist.get_mut(&start).unwrap() = 0;
    heap.push(DijkstraState {
        cost: 0,
        current: start,
    });
    // keep track of which new keys we can reach
    let mut reach = HashSet::new();

    while let Some(DijkstraState { cost, current }) = heap.pop() {
        if current.is_lowercase() && !keys.contains(&current) {
            reach.insert(current);
            continue;
        }

        // Important as we may have already found a better way
        if cost > dist[&current] {
            continue;
        }

        for (&next_node, &next_cost) in graph.get(&current).unwrap().iter() {
            // check if we have permission to pass
            if next_node.is_uppercase() && !keys.contains(&next_node.to_ascii_lowercase()) {
                continue;
            }

            let next = DijkstraState {
                cost: cost + next_cost,
                current: next_node,
            };

            if next.cost < dist[&next_node] {
                dist.insert(next_node, next.cost);
                heap.push(next);
            }
        }
    }
    // return a tuple of (new keys, cost to reach)
    reach.into_iter().map(|node| (node, dist[&node])).collect()
}

// modify grid to split map into 4 sections
// add 4 robots on each section
fn four_robots(grid: &mut HashMap<IVec2, Tile>) {
    let robot_coord = grid
        .iter()
        .find(|(_, &v)| v == Tile::Node('@'))
        .map(|(k, _)| *k)
        .unwrap();

    grid.insert(robot_coord, Tile::Wall);
    for neighbor in neighbors(robot_coord) {
        grid.insert(neighbor, Tile::Wall);
    }
    grid.insert(
        IVec2::new(robot_coord.x - 1, robot_coord.y - 1),
        Tile::Node('@'),
    );
    grid.insert(
        IVec2::new(robot_coord.x - 1, robot_coord.y + 1),
        Tile::Node('='),
    );

    grid.insert(
        IVec2::new(robot_coord.x + 1, robot_coord.y + 1),
        Tile::Node('%'),
    );
    grid.insert(
        IVec2::new(robot_coord.x + 1, robot_coord.y - 1),
        Tile::Node('$'),
    );
}

fn search_four(graph: HashMap<char, HashMap<char, usize>>) -> usize {
    let mut priority_queue = BinaryHeap::new();
    let key_count = graph.iter().filter(|(k, _)| k.is_lowercase()).count();

    // keep track of best cost at (robot_positions, keys collected)
    let mut distances: HashMap<([char; 4], BTreeSet<char>), usize> = HashMap::new();
    let robots = ['@', '=', '%', '$'];

    distances.insert((robots, BTreeSet::new()), 0);

    let start = FourState {
        steps: 0,
        robots,
        keys: BTreeSet::new(),
    };

    priority_queue.push(start);

    // search keys cache, avoid recomputing search keys for the same (position, keys collected)
    let mut cache = HashMap::new();

    while let Some(current) = priority_queue.pop() {
        if current.keys.len() == key_count {
            return current.steps;
        }

        if let Some(&best_steps) = distances.get(&(current.robots, current.keys.clone())) {
            if current.steps > best_steps {
                continue;
            }
        }

        for (robot_number, &robot_location) in current.robots.iter().enumerate() {
            let cache_key = (robot_location, current.keys.clone());

            let cached_entry = cache
                .entry(cache_key)
                .or_insert_with(|| search_keys(&graph, &current.keys, robot_location));

            for &(next_node, cost) in cached_entry.iter() {
                let mut next_keys = current.keys.clone();
                next_keys.insert(next_node);

                let mut next_robots = current.robots;
                next_robots[robot_number] = next_node;

                let next_steps = current.steps + cost;

                let distances_entry = distances
                    .entry((next_robots, next_keys.clone()))
                    .or_insert(usize::max_value());

                if next_steps < *distances_entry {
                    *distances_entry = next_steps;
                    let next_state = FourState {
                        steps: next_steps,
                        robots: next_robots,
                        keys: next_keys,
                    };

                    priority_queue.push(next_state);
                }
            }
        }
    }
    // no path found
    usize::max_value()
}

#[derive(PartialEq, Eq)]
struct FourState {
    steps: usize,
    robots: [char; 4],
    keys: BTreeSet<char>,
}

impl Ord for FourState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .steps
            .cmp(&self.steps)
            .then(self.keys.len().cmp(&other.keys.len()))
    }
}

impl PartialOrd for FourState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse_grid(input);
    let graph = graph(&grid);

    Some(search(graph, '@'))
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = parse_grid(input);
    four_robots(&mut grid);
    let graph = graph(&grid);

    Some(search_four(graph))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(8));
    }
}
