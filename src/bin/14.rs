use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(14);

#[derive(Debug)]
struct Chemical {
    name: String,
    amount: u64,
}

#[derive(Debug)]
struct Reaction {
    inputs: Vec<Chemical>,
    output: Chemical,
}

fn parse_input(input: &str) -> Vec<Reaction> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(" => ");
            let inputs = parts
                .next()
                .unwrap()
                .split(", ")
                .map(|input| {
                    let mut parts = input.split(' ');
                    let amount = parts.next().unwrap().parse().unwrap();
                    let name = parts.next().unwrap().to_string();
                    Chemical { name, amount }
                })
                .collect();
            let output = parts
                .next()
                .map(|output| {
                    let mut parts = output.split(' ');
                    let amount = parts.next().unwrap().parse().unwrap();
                    let name = parts.next().unwrap().to_string();
                    Chemical { name, amount }
                })
                .unwrap();
            Reaction { inputs, output }
        })
        .collect()
}

fn calculate_ore(reactions: &[Reaction], fuel: u64) -> u64 {
    let mut to_process = VecDeque::from([Chemical {
        name: "FUEL".to_string(),
        amount: fuel,
    }]);

    let mut excess = HashMap::new();

    let mut ore = 0;

    while let Some(chemical) = to_process.pop_front() {
        let reaction = reactions
            .iter()
            .find(|reaction| reaction.output.name == chemical.name)
            .unwrap();

        let mut amount = chemical.amount;
        if let Some(excess_amount) = excess.get_mut(&chemical.name) {
            if *excess_amount >= amount {
                *excess_amount -= amount;
                continue;
            } else {
                amount -= *excess_amount;
                *excess_amount = 0;
            }
        }

        let reaction_count = (amount as f64 / reaction.output.amount as f64).ceil() as u64;
        let excess_amount = reaction_count * reaction.output.amount - amount;
        *excess.entry(chemical.name.clone()).or_insert(0) += excess_amount;

        for input in &reaction.inputs {
            if input.name == "ORE" {
                ore += input.amount * reaction_count;
            } else {
                to_process.push_back(Chemical {
                    name: input.name.clone(),
                    amount: input.amount * reaction_count,
                });
            }
        }
    }

    ore
}

pub fn part_one(input: &str) -> Option<u64> {
    let reactions = parse_input(input);

    let ore = calculate_ore(&reactions, 1);

    Some(ore)
}

pub fn part_two(input: &str) -> Option<u64> {
    let reactions = parse_input(input);
    let one_fuel = calculate_ore(&reactions, 1);

    let mut tried_fuels = HashSet::new();

    let mut try_fuel = 1_000_000_000_000 / one_fuel;

    loop {
        let ore = calculate_ore(&reactions, try_fuel);
        if ore > 1_000_000_000_000 {
            break;
        }

        tried_fuels.insert(try_fuel);
        try_fuel *= 2;
    }

    let mut lower = try_fuel / 2;
    let mut upper = try_fuel;

    while lower < upper {
        let mid = (lower + upper) / 2;
        if tried_fuels.contains(&mid) {
            break;
        }

        let ore = calculate_ore(&reactions, mid);
        if ore > 1_000_000_000_000 {
            upper = mid;
        } else {
            lower = mid + 1;
        }
    }

    Some(lower - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82892753));
    }
}
