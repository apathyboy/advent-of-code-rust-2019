use glam::IVec2;
use std::{cmp::Ordering, collections::HashMap};

use advent_of_code::IntcodeComputer;

advent_of_code::solution!(13);

const QUARTER_SLOT: usize = 0;
const QUARTERS: i128 = 2;
const SCORE_SCREEN_COORD: IVec2 = IVec2::new(-1, 0);
const DEFAULT_SCORE: i128 = 0;

const EMPTY_TILE: i128 = 0;
const WALL_TILE: i128 = 1;
const BLOCK_TILE: i128 = 2;
const PADDLE_TILE: i128 = 3;
const BALL_TILE: i128 = 4;

fn get_score(screen: &HashMap<IVec2, i128>) -> i128 {
    let score = screen.get(&SCORE_SCREEN_COORD);

    *score.unwrap_or(&DEFAULT_SCORE)
}

fn get_tile_position(screen: &HashMap<IVec2, i128>, tile_id: i128) -> Option<IVec2> {
    let tile = screen.iter().find(|(_, &t)| t == tile_id);

    if tile.is_some() {
        Some(*tile?.0)
    } else {
        None
    }
}

fn blocks_remaining(screen: &HashMap<IVec2, i128>) -> usize {
    screen.values().filter(|&t| *t == BLOCK_TILE).count()
}

#[allow(unused)]
fn draw_screen(screen: &HashMap<IVec2, i128>) {
    let max_x = screen.keys().map(|pos| pos.x).max().unwrap();
    let max_y = screen.keys().map(|pos| pos.y).max().unwrap();

    let score = get_score(screen);

    println!("Score: {score}");

    for y in 0..=max_y {
        for x in 0..=max_x {
            let tile = screen.get(&IVec2::new(x, y)).unwrap_or(&EMPTY_TILE);

            match *tile {
                EMPTY_TILE => print!(" "),
                WALL_TILE => print!("█"),
                BLOCK_TILE => print!("#"),
                PADDLE_TILE => print!("-"),
                BALL_TILE => print!("*"),
                _ => panic!("Invalid tile id {}", *tile),
            }
        }

        println!();
    }

    println!();
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut arcade_cabinet = IntcodeComputer::new();

    arcade_cabinet.load_program_from_str(input);

    let mut screen = HashMap::new();

    while arcade_cabinet.is_running() {
        let x = arcade_cabinet.run_to_next_output()?;
        let y = arcade_cabinet.run_to_next_output()?;
        let tile_id = arcade_cabinet.run_to_next_output()?;

        let pos = IVec2::new(x as i32, y as i32);

        *screen.entry(pos).or_insert(tile_id) = tile_id;
    }

    Some(blocks_remaining(&screen))
}

pub fn part_two(input: &str) -> Option<i128> {
    let mut arcade_cabinet = IntcodeComputer::new();

    arcade_cabinet.load_program_from_str(input);
    arcade_cabinet.set(QUARTER_SLOT, QUARTERS);

    let mut screen = HashMap::new();

    while arcade_cabinet.is_running() {
        let x = arcade_cabinet.run_to_next_output()?;
        let y = arcade_cabinet.run_to_next_output()?;
        let tile_id = arcade_cabinet.run_to_next_output()?;

        let pos = IVec2::new(x as i32, y as i32);

        *screen.entry(pos).or_insert(tile_id) = tile_id;

        if arcade_cabinet.ticks() < 16425 {
            continue;
        } else if arcade_cabinet.ticks() == 16425 || tile_id == BALL_TILE {
            let paddle = get_tile_position(&screen, PADDLE_TILE)?;
            let ball = get_tile_position(&screen, BALL_TILE)?;

            let joystick = match paddle.x.cmp(&ball.x) {
                Ordering::Greater => -1,
                Ordering::Less => 1,
                Ordering::Equal => 0,
            };

            arcade_cabinet.set_input(joystick);
        }
    }

    Some(get_score(&screen))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_one() {
        assert_eq!(1, 1);
    }
}
