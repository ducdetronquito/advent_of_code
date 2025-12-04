use std::{collections::HashMap, fs};

fn main() {
    let mut map = parse_input("src/input");
    let removed_rolls = map.remove_accessible_rolls();
    dbg!(removed_rolls); // 1540

    let mut total_removed_rolls = removed_rolls;
    loop {
        let removed_rolls = map.remove_accessible_rolls();
        total_removed_rolls += removed_rolls;
        if removed_rolls == 0 {
            break;
        }
    }

    dbg!(total_removed_rolls); // 8972
}

#[derive(Debug)]
struct Map(HashMap<Point, char>);

impl Map {
    fn remove_accessible_rolls(&mut self) -> usize {
        let mut accessible_rolls: Vec<Point> = vec![];
        for (point, value) in self.0.iter() {
            if *value != '@' {
                continue;
            }

            let neighbor_rolls = point
                .neighors()
                .iter()
                .map(|neighbor| match self.0.get(neighbor) {
                    Some(value) if *value == '@' => 1,
                    _ => 0,
                })
                .sum::<u64>();
            if neighbor_rolls < 4 {
                accessible_rolls.push(Point {
                    x: point.x,
                    y: point.y,
                });
            }
        }

        let result = accessible_rolls.len();
        for accessible_roll in accessible_rolls {
            self.0.insert(accessible_roll, '.');
        }

        result
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn neighors(&self) -> [Point; 8] {
        [
            Point {
                x: self.x - 1,
                y: self.y - 1,
            },
            Point {
                x: self.x,
                y: self.y - 1,
            },
            Point {
                x: self.x + 1,
                y: self.y - 1,
            },
            Point {
                x: self.x - 1,
                y: self.y,
            },
            Point {
                x: self.x + 1,
                y: self.y,
            },
            Point {
                x: self.x - 1,
                y: self.y + 1,
            },
            Point {
                x: self.x,
                y: self.y + 1,
            },
            Point {
                x: self.x + 1,
                y: self.y + 1,
            },
        ]
    }
}

fn parse_input(path: &str) -> Map {
    let content = fs::read_to_string(path).unwrap();
    let mut map = HashMap::new();

    for (y, line) in content.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            map.insert(
                Point {
                    x: x as i64,
                    y: y as i64,
                },
                char,
            );
        }
    }

    Map(map)
}
