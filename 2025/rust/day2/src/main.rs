use std::fs;

fn main() {
    let ids: Vec<u64> = parse_input("./src/input")
        .iter()
        .flat_map(|range| range.start..=range.end)
        .collect();

    let step1_invalid_ids: u64 = ids
        .iter()
        .filter(|value| {
            let string_value = value.to_string();
            let pattern_size = string_value.len().div_euclid(2);
            if string_value.len().rem_euclid(2) != 0 {
                return false;
            }
            has_repeated_pattern(&string_value, pattern_size)
        })
        .sum();

    dbg!(step1_invalid_ids); // 40055209690

    let step2_invalid_ids: u64 = ids
        .iter()
        .filter(|value| {
            let string_value = value.to_string();
            if string_value.len() <= 1 {
                return false;
            }
            let max_pattern_size = string_value.len().div_euclid(2);

            (1..=max_pattern_size)
                .any(|pattern_size| has_repeated_pattern(&string_value, pattern_size))
        })
        .sum();

    dbg!(step2_invalid_ids); // 50857215650
}

fn has_repeated_pattern(string: &str, pattern_size: usize) -> bool {
    if pattern_size >= string.len() || pattern_size == 0 {
        return false;
    }

    let pattern_count = string.len().div_euclid(pattern_size);
    if pattern_count < 2 {
        return false;
    }

    string[0..pattern_size].repeat(pattern_count) == string
}

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

fn parse_input(path: &str) -> Vec<Range> {
    fs::read_to_string(path)
        .unwrap()
        .split(",")
        .map(|item| {
            let elements = item.split("-").collect::<Vec<_>>();
            let start = elements[0].parse().expect("Invalid start value");
            let end = elements[1].parse().expect("Invalid end value");
            Range { start, end }
        })
        .collect::<Vec<_>>()
}
