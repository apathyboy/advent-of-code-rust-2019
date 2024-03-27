advent_of_code::solution!(22);

fn deal_into_new_stack(deck: &mut [u32]) {
    deck.reverse();
}

fn cut(deck: &mut Vec<u32>, n: i32) {
    if n > 0 {
        let mut cut = deck.split_off(n as usize);
        cut.append(deck);
        *deck = cut;
    } else {
        let n = n.unsigned_abs() as usize;
        let mut cut = deck.split_off(deck.len() - n);
        cut.append(deck);
        *deck = cut;
    }
}

fn deal_with_increment(deck: &mut Vec<u32>, n: i32) {
    let mut new_deck = vec![0; deck.len()];
    let mut i = 0;
    for card in deck.iter() {
        new_deck[i] = *card;
        i += n as usize;
        i = i.rem_euclid(deck.len());
    }
    *deck = new_deck;
}

fn shuffle(deck: &mut Vec<u32>, instructions: &str) {
    for instruction in instructions.lines() {
        if instruction.starts_with("deal into new stack") {
            deal_into_new_stack(deck);
        } else if instruction.starts_with("cut") {
            let n = instruction
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();
            cut(deck, n);
        } else if instruction.starts_with("deal with increment") {
            let n = instruction
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();
            deal_with_increment(deck, n);
        }
    }
}

fn modinv(mut a: i128, mut base: i128) -> i128 {
    if base == 1 {
        return 0;
    }

    let orig = base;

    let mut x = 1;
    let mut y = 0;

    while a > 1 {
        let q = a / base;
        let tmp = base;
        base = a % base;
        a = tmp;
        let tmp = y;
        y = x - q * y;
        x = tmp;
    }

    if x < 0 {
        x + orig
    } else {
        x
    }
}

fn modp(b: i128, exp: i128, base: i128) -> i128 {
    let mut x = 1;
    let mut p = b % base;

    for i in 0..128 {
        if 1 & (exp >> i) == 1 {
            x = x * p % base;
        }

        p = p * p % base;
    }

    x
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut deck: Vec<u32> = (0..10007).collect();
    shuffle(&mut deck, input);
    deck.iter().position(|&x| x == 2019).map(|x| x as u32)
}

pub fn part_two(input: &str) -> Option<i128> {
    const D: i128 = 119_315_717_514_047;
    const N: i128 = 101_741_582_076_661;
    const TGT: i128 = 2020;

    let mut a = 1;
    let mut b = 0;

    for instruction in input.lines().rev() {
        if instruction.starts_with("deal into new stack") {
            b += 1;
            b *= -1;
            a *= -1;
        } else if instruction.starts_with("cut") {
            let n: i128 = instruction
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();
            b += if n < 0 { n + D } else { n };
        } else if instruction.starts_with("deal with increment") {
            let n: i128 = instruction
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();
            let inv = modinv(n, D);
            a = a * inv % D;
            b = b * inv % D;
        }

        a %= D;
        b %= D;

        if a < 0 {
            a += D;
        }

        if b < 0 {
            b += D;
        }
    }

    let i1 = modp(a, N, D) * TGT % D;
    let i2 = (modp(a, N, D) + D - 1) % D;
    let i3 = b * i2 % D;
    let i4 = modp(a - 1, D - 2, D);
    let ans = (i1 + i3 * i4) % D;

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut deck: Vec<u32> = (0..10).collect();
        shuffle(
            &mut deck,
            &advent_of_code::template::read_file("examples", DAY),
        );
        assert_eq!(deck, Vec::from([9, 2, 5, 8, 1, 4, 7, 0, 3, 6]));
    }
}
