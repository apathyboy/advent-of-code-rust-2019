use advent_of_code::IntcodeComputer;
use glam::IVec2;
use std::collections::HashMap;

advent_of_code::solution!(11);

struct Robot {
    position: IVec2,
    direction: IVec2,
    brain: IntcodeComputer,
    panels: HashMap<IVec2, i128>,
}

impl Robot {
    fn new(input: &str) -> Self {
        let mut brain = IntcodeComputer::new();
        brain.load_program_from_str(input);

        Self {
            position: IVec2::ZERO,
            direction: IVec2::Y,
            brain,
            panels: HashMap::new(),
        }
    }

    fn turn_left(&mut self) {
        self.direction = IVec2::new(-self.direction.y, self.direction.x);
    }

    fn turn_right(&mut self) {
        self.direction = IVec2::new(self.direction.y, -self.direction.x);
    }

    fn move_forward(&mut self) {
        self.position += self.direction;
    }

    fn paint(&mut self) {
        let panel = self.panels.entry(self.position).or_insert(0);

        self.brain.set_input(*panel);

        *panel = self.brain.run_to_next_output().unwrap();

        match self.brain.run_to_next_output().unwrap() {
            0 => self.turn_left(),
            1 => self.turn_right(),
            _ => panic!("Invalid turn direction"),
        }

        self.move_forward();
    }

    fn run(&mut self) {
        while self.brain.is_running() {
            self.paint();
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut robot = Robot::new(input);

    robot.run();

    Some(robot.panels.len())
}

pub fn part_two(input: &str) -> Option<String> {
    let mut robot = Robot::new(input);

    robot.panels.insert(IVec2::ZERO, 1);

    robot.run();

    let min_x = robot.panels.keys().map(|pos| pos.x).min().unwrap();
    let max_x = robot.panels.keys().map(|pos| pos.x).max().unwrap();
    let min_y = robot.panels.keys().map(|pos| pos.y).min().unwrap();
    let max_y = robot.panels.keys().map(|pos| pos.y).max().unwrap();

    let mut result = String::new();

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            let panel = robot.panels.get(&IVec2::new(x, y)).unwrap_or(&0);

            result.push(if *panel == 1 { '█' } else { ' ' });
        }

        result.push('\n');
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_one() {
        assert_eq!(1, 1);
    }
}
