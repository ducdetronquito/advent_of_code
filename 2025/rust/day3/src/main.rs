use std::fs;

fn main() {
    let banks = parse_input("./src/input");

    let low_joltage: u64 = banks.iter().map(|bank| joltage(bank, 2)).sum();
    dbg!(low_joltage); // 17694

    let high_joltage: u64 = banks.iter().map(|bank| joltage(bank, 12)).sum();

    dbg!(high_joltage); // 175659236361660
}

fn joltage(bank: &Bank, number_of_battery: u32) -> u64 {
    let mut result: u64 = 0;
    let mut start: usize = 0;

    for number_of_battery in (0..=number_of_battery - 1).rev() {
        let end = bank.len() - number_of_battery as usize;
        let max = find_max_with_position(&bank[start..end]).unwrap();
        result += max.0 * 10_u64.pow(number_of_battery);
        start += max.1 + 1;
    }

    result
}

fn parse_bank(input: &str) -> Bank {
    input
        .bytes()
        .map(|value| (value - 48) as u64)
        .collect::<Bank>()
}

fn find_max_with_position(values: &[u64]) -> Option<(u64, usize)> {
    if values.is_empty() {
        return None;
    }

    let mut max = 0;
    let mut max_position = 0;
    for (i, value) in values.iter().enumerate() {
        if *value > max {
            max = *value;
            max_position = i;
        }
    }

    Some((max, max_position))
}

type Bank = Vec<u64>;

fn parse_input(path: &str) -> Vec<Bank> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(parse_bank)
        .collect()
}
