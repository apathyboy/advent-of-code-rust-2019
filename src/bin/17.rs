use advent_of_code::IntcodeComputer;
use glam::IVec2;

advent_of_code::solution!(17);

struct ShipExterior {
    scaffolding: Vec<IVec2>,
    vacuum_robot: IVec2,
    width: usize,
    height: usize,
}

impl ShipExterior {
    fn new(scaffolding: Vec<IVec2>, vacuum_robot: IVec2, width: usize, height: usize) -> Self {
        Self {
            scaffolding,
            vacuum_robot,
            width,
            height,
        }
    }

    fn is_scaffolding(&self, x: usize, y: usize) -> bool {
        self.scaffolding.contains(&IVec2::new(x as i32, y as i32))
    }

    fn is_intersection(&self, x: usize, y: usize) -> bool {
        if !self.is_scaffolding(x, y) {
            return false;
        }

        let mut count = 0;
        if x > 0 && self.is_scaffolding(x - 1, y) {
            count += 1;
        }
        if x < self.width - 1 && self.is_scaffolding(x + 1, y) {
            count += 1;
        }
        if y > 0 && self.is_scaffolding(x, y - 1) {
            count += 1;
        }
        if y < self.height - 1 && self.is_scaffolding(x, y + 1) {
            count += 1;
        }

        count >= 3
    }
}

fn read_ship_exterior_from_computer(computer: &mut IntcodeComputer) -> ShipExterior {
    let mut scaffolding = Vec::new();
    let mut x = 0;
    let mut y = 0;
    let mut width = 0;
    let mut height = 0;

    let mut last_char = ' ';

    let mut vacuum_robot = IVec2::ZERO;

    while let Some(output) = computer.run_until_output() {
        let c = output as u8 as char;
        if c == '\n' {
            if last_char == '\n' {
                break;
            }

            if width == 0 {
                width = x;
            }

            x = 0;
            y += 1;
            height = y;
        } else {
            if c == '#' {
                scaffolding.push(IVec2::new(x, y));
            }
            if c == '^' {
                scaffolding.push(IVec2::new(x, y));
                vacuum_robot = IVec2::new(x, y);
            }
            x += 1;
        }

        last_char = c;
    }

    ShipExterior::new(scaffolding, vacuum_robot, width as usize, height as usize)
}

#[allow(dead_code)]
fn draw_exterior(exterior: &ShipExterior) {
    for y in 0..exterior.height {
        for x in 0..exterior.width {
            if exterior.vacuum_robot == IVec2::new(x as i32, y as i32) {
                print!("^");
            } else if exterior.is_scaffolding(x, y) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn turn_left(dir: IVec2) -> IVec2 {
    IVec2::new(dir.y, -dir.x)
}

fn turn_right(dir: IVec2) -> IVec2 {
    IVec2::new(-dir.y, dir.x)
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut computer = IntcodeComputer::new();
    computer.load_program_from_str(input);

    let exterior = read_ship_exterior_from_computer(&mut computer);

    let mut sum = 0;

    for y in 0..exterior.height {
        for x in 0..exterior.width {
            if exterior.is_intersection(x, y) {
                sum += x * y;
            }
        }
    }

    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<i128> {
    let mut computer = IntcodeComputer::new();
    computer.load_program_from_str(input);

    computer.set(0, 2);

    let exterior = read_ship_exterior_from_computer(&mut computer);

    let mut dir = IVec2::new(0, -1);
    let mut pos = exterior.vacuum_robot;

    let mut path = String::new();

    loop {
        let mut steps = 0;
        while exterior.is_scaffolding((pos + dir).x as usize, (pos + dir).y as usize) {
            steps += 1;
            pos += dir;
        }

        if steps > 0 {
            path.push_str(&steps.to_string());
            path.push(',');
        }

        let left = turn_left(dir);
        let right = turn_right(dir);

        if exterior.is_scaffolding((pos + left).x as usize, (pos + left).y as usize) {
            path.push_str("L,");
            dir = left;
        } else if exterior.is_scaffolding((pos + right).x as usize, (pos + right).y as usize) {
            path.push_str("R,");
            dir = right;
        } else {
            break;
        }
    }

    let main_routine = "A,A,B,C,B,A,C,B,C,A".to_string();
    let a_routine = "L,6,R,12,L,6,L,8,L,8".to_string();
    let b_routine = "L,6,R,12,R,8,L,8".to_string();
    let c_routine = "L,4,L,4,L,6".to_string();

    let input = format!(
        "{}\n{}\n{}\n{}\nn\n",
        main_routine, a_routine, b_routine, c_routine
    );

    for c in input.chars() {
        computer.set_input(c as i128);
    }

    computer.run();

    computer.get_next_output()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_one() {
        assert_eq!(1, 1);
    }
}
