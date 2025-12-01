use std::fs;

fn main() {
    let rotations: Vec<Rotation> = parse_input("./src/input");
    let mut position_ends_on_zero = 0;
    let mut clicks_on_zero = 0;

    rotations.iter().fold(50, |position: i32, rotation| {
        let mut next_position = match rotation {
            Rotation::Left(distance) => position - distance,
            Rotation::Right(distance) => position + distance,
        };

        clicks_on_zero += count_click_on_zero(position, next_position);
        next_position = next_position.rem_euclid(100);

        if next_position == 0 {
            position_ends_on_zero += 1;
        }

        next_position
    });

    dbg!(position_ends_on_zero); // 1007
    dbg!(clicks_on_zero); // 5820
}

#[derive(Debug)]
enum Rotation {
    Left(i32),
    Right(i32),
}

fn parse_line(line: &str) -> Rotation {
    if let Some(distance) = line.strip_prefix("L") {
        return Rotation::Left(distance.parse().unwrap());
    }
    if let Some(distance) = line.strip_prefix("R") {
        return Rotation::Right(distance.parse().unwrap());
    }

    panic!("Unexpected line format")
}

fn parse_input(path: &str) -> Vec<Rotation> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(parse_line)
        .collect()
}

fn count_click_on_zero(position: i32, next_position: i32) -> i32 {
    let step = if next_position >= position { 1 } else { -1 };

    let mut i = position;
    let mut result = 0;

    while i != next_position {
        i += step;
        if i.rem_euclid(100) == 0 {
            result += 1;
        }
    }

    result
}
