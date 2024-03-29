use std::collections::{HashMap, HashSet};

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

type Cell = (isize, isize, isize);

struct Grid {
    rows: isize,
    cols: isize,
    bugs: HashSet<Cell>,
}

const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

impl Grid {
    fn from_str(input: &str) -> Option<Grid> {
        let (mut bugs, mut rows, mut cols) = (HashSet::new(), 0isize, 0isize);

        for (r, row) in input.lines().enumerate() {
            for (c, cell) in row.chars().enumerate() {
                cols = cols.max(c as isize + 1);
                rows = rows.max(r as isize + 1);
                if cell == '#' {
                    bugs.insert((r as isize, c as isize, 0));
                }
            }
        }

        Some(Grid { rows, cols, bugs })
    }

    fn adj_cells(&self, cell: &Cell) -> Vec<Cell> {
        let (r, c, l) = cell;

        let mut adj = Vec::new();

        for (dr, dc) in &DIRS {
            let (nr, nc, nl) = (r + dr, c + dc, *l);

            if nc == self.cols / 2 && nr == self.rows / 2 {
                if *dr == 0 {
                    for nr in 0..self.rows {
                        adj.push(match dc {
                            1 => (nr, 0, nl - 1),
                            _ => (nr, self.cols - 1, nl - 1),
                        });
                    }
                } else {
                    for nc in 0..self.cols {
                        adj.push(match dr {
                            1 => (0, nc, nl - 1),
                            _ => (self.rows - 1, nc, nl - 1),
                        });
                    }
                }
            } else if nr < 0 || nc < 0 || nr >= self.rows || nc >= self.cols {
                adj.push((self.rows / 2 + dr, self.cols / 2 + dc, l + 1));
            } else {
                adj.push((nr, nc, nl));
            }
        }

        adj
    }

    fn adj_bugs_count(&self, cell: &Cell) -> usize {
        self.adj_cells(cell)
            .into_iter()
            .filter(|cell| self.bugs.contains(cell))
            .count()
    }

    fn tick(&mut self) {
        let mut bugs = HashSet::new();

        for bug in &self.bugs {
            if self.adj_bugs_count(bug) == 1 {
                bugs.insert(*bug);
            }

            for infested in self.adj_cells(bug) {
                if !&self.bugs.contains(&infested) {
                    let count = self.adj_bugs_count(&infested);
                    if count == 1 || count == 2 {
                        bugs.insert(infested);
                    }
                }
            }
        }

        self.bugs = bugs;
    }

    fn count(&self) -> usize {
        self.bugs.len()
    }
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
    let mut grid = Grid::from_str(input).unwrap();
    for _ in 0..200 {
        grid.tick();
    }

    Some(grid.count() as u32)
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
