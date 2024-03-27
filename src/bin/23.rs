use std::collections::HashSet;

use advent_of_code::IntcodeComputer;

advent_of_code::solution!(23);

struct NetworkInterfaceController {
    computer: IntcodeComputer,
}

struct NotAlwaysTransmitting {
    x: i128,
    y: i128,
}

pub fn part_one(input: &str) -> Option<i128> {
    let mut nics = Vec::new();

    for network_address in 0..50 {
        let mut computer = IntcodeComputer::new();
        computer.load_program_from_str(input);
        computer.set_default_input(-1);
        computer.set_input(network_address);
        computer.run_until_io();

        nics.push(NetworkInterfaceController { computer });
    }

    loop {
        let outputs: Vec<Option<(usize, i128, i128)>> = nics
            .iter_mut()
            .map(|nic| {
                nic.computer.run_until_io();

                if nic.computer.has_output() {
                    let address = nic.computer.get_next_output()?;
                    let x = nic.computer.run_until_output()?;
                    let y = nic.computer.run_until_output()?;

                    Some((address as usize, x, y))
                } else {
                    None
                }
            })
            .collect();

        for packet in outputs {
            if packet.is_none() {
                continue;
            }

            let (address, x, y) = packet.unwrap();

            if address == 255 {
                return Some(y);
            }

            nics[address].computer.set_input(x);
            nics[address].computer.set_input(y);
        }
    }
}

pub fn part_two(input: &str) -> Option<i128> {
    let mut nics = Vec::new();
    let mut nat = NotAlwaysTransmitting { x: 0, y: 0 };
    let mut nat_delivered_y = HashSet::new();

    for network_address in 0..50 {
        let mut computer = IntcodeComputer::new();
        computer.load_program_from_str(input);
        computer.set_default_input(-1);
        computer.set_input(network_address);
        computer.run_until_io();

        nics.push(NetworkInterfaceController { computer });
    }

    loop {
        let outputs: Vec<Option<(usize, i128, i128)>> = nics
            .iter_mut()
            .map(|nic| {
                nic.computer.run_until_io();

                if nic.computer.has_output() {
                    let address = nic.computer.get_next_output()?;
                    let x = nic.computer.run_until_output()?;
                    let y = nic.computer.run_until_output()?;

                    Some((address as usize, x, y))
                } else {
                    None
                }
            })
            .collect();

        for packet in outputs {
            if packet.is_none() {
                continue;
            }

            let (address, x, y) = packet.unwrap();

            if address == 255 {
                nat.x = x;
                nat.y = y;

                if nics.iter().all(|nic| !nic.computer.has_input()) {
                    nics[0].computer.set_input(nat.x);
                    nics[0].computer.set_input(nat.y);

                    if nat_delivered_y.contains(&nat.y) {
                        return Some(y);
                    } else {
                        nat_delivered_y.insert(nat.y);
                    }
                }

                continue;
            }

            nics[address].computer.set_input(x);
            nics[address].computer.set_input(y);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_one() {
        assert_eq!(1, 1);
    }
}
