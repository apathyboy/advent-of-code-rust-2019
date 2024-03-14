use glam::IVec3;
use std::cmp::Ordering;

advent_of_code::solution!(12);

#[derive(Debug, Clone, Copy)]
struct Moon {
    position: IVec3,
    velocity: IVec3,
}

fn compare_axis(a: i32, b: i32) -> i32 {
    match a.cmp(&b) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    }
}

impl Moon {
    fn new() -> Self {
        Self {
            position: IVec3::ZERO,
            velocity: IVec3::ZERO,
        }
    }

    fn apply_gravity(&mut self, other: Moon) {
        self.velocity.x += compare_axis(self.position.x, other.position.x);
        self.velocity.y += compare_axis(self.position.y, other.position.y);
        self.velocity.z += compare_axis(self.position.z, other.position.z);
    }

    fn apply_velocity(&mut self) {
        self.position += self.velocity;
    }

    fn energy(&self) -> i32 {
        let potential_energy =
            self.position.x.abs() + self.position.y.abs() + self.position.z.abs();
        let kinetic_energy = self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs();

        potential_energy * kinetic_energy
    }
}

fn parse_moon(line: &str) -> Option<Moon> {
    let mut moon = Moon::new();

    let axis = line[1..line.len() - 1].split(", ").collect::<Vec<_>>();

    moon.position.x = axis[0][2..].parse().ok()?;
    moon.position.y = axis[1][2..].parse().ok()?;
    moon.position.z = axis[2][2..].parse().ok()?;

    Some(moon)
}

fn parse(input: &str) -> Vec<Moon> {
    input.lines().filter_map(parse_moon).collect()
}

fn simulate(moons: &mut [Moon], total_steps: usize) -> i32 {
    let mut steps = 0;

    while steps < total_steps {
        for i in 0..moons.len() {
            for j in i + 1..moons.len() {
                let moon1 = moons[i];
                let moon2 = moons[j];

                moons[i].apply_gravity(moon2);
                moons[j].apply_gravity(moon1);
            }
        }

        for moon in moons.iter_mut() {
            moon.apply_velocity();
        }

        steps += 1;
    }

    count_energy(moons)
}

fn count_energy(moons: &[Moon]) -> i32 {
    moons.iter().map(|m| m.energy()).sum()
}

fn velocity_diff(positions: &[i32]) -> Vec<i32> {
    positions
        .iter()
        .enumerate()
        .map(|(idx, &pos1)| {
            positions.iter().enumerate().fold(0, |acc, (jdx, &pos2)| {
                if idx == jdx {
                    acc
                } else {
                    acc + match pos1.cmp(&pos2) {
                        Ordering::Less => 1,
                        Ordering::Greater => -1,
                        Ordering::Equal => 0,
                    }
                }
            })
        })
        .collect()
}

fn find_steps_axis(mut positions: Vec<i32>) -> u64 {
    let mut velocities = vec![0; positions.len()];

    let mut steps = 0;
    loop {
        let velocity_change = velocity_diff(&positions);
        for (v, change) in velocities.iter_mut().zip(velocity_change) {
            *v += change;
        }
        for (p, v) in positions.iter_mut().zip(velocities.iter()) {
            *p += v;
        }

        steps += 1;

        if velocities.iter().all(|&v| v == 0) {
            break;
        }
    }
    steps * 2
}

fn gcd(x: u64, y: u64) -> u64 {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn lcm3(a: u64, b: u64, c: u64) -> u64 {
    lcm(a, lcm(b, c))
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut moons = parse(input);

    Some(simulate(&mut moons, 1000))
}

pub fn part_two(input: &str) -> Option<u64> {
    let moons = parse(input);

    let x = find_steps_axis(moons.iter().map(|m| m.position.x).collect());
    let y = find_steps_axis(moons.iter().map(|m| m.position.y).collect());
    let z = find_steps_axis(moons.iter().map(|m| m.position.z).collect());

    Some(lcm3(x, y, z))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let mut moons = parse(input);

        let result = simulate(&mut moons, 10);

        assert_eq!(result, 179);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
