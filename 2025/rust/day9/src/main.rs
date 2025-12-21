use std::fs;

use itertools::Itertools;

fn main() {
    let tiles = parse("./src/input");

    let largest_area = unique_pairs(&tiles)
        .iter()
        .map(|(tile, other_tile)| rectangle_area(tile, other_tile))
        .sorted()
        .rev()
        .collect::<Vec<usize>>()
        .first()
        .unwrap()
        .clone();
    dbg!(largest_area); // 4760959496
}

fn unique_pairs(tiles: &[Tile]) -> Vec<(Tile, Tile)> {
    let mut result = vec![];
    for (index, tile) in tiles.iter().enumerate() {
        for other_tile in tiles.iter().skip(index + 1) {
            result.push((tile.clone(), other_tile.clone()))
        }
    }

    result
}

fn rectangle_area(tile: &Tile, other_tile: &Tile) -> usize {
    (other_tile.x.abs_diff(tile.x) + 1) * (other_tile.y.abs_diff(tile.y) + 1)
}

#[derive(Debug, Clone)]
struct Tile {
    x: usize,
    y: usize,
}

fn parse(path: &str) -> Vec<Tile> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            let values: Vec<usize> = line
                .split(",")
                .map(|value| value.parse().unwrap())
                .collect();

            Tile {
                x: values[0],
                y: values[1],
            }
        })
        .collect()
}
