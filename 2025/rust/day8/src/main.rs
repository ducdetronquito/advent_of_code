use itertools::Itertools;
use std::{collections::HashSet, fs, hash::Hash};

fn main() {
    let junction_boxes = parse("./src/input");
    let product_of_3_largest_circuits_size = largest_circuits(&junction_boxes, 3)
        .iter()
        .map(|circuit| circuit.len())
        .product::<usize>();
    dbg!(product_of_3_largest_circuits_size); // 140008
}

fn unique_pairs(values: &[JunctionBox]) -> Vec<(JunctionBox, JunctionBox)> {
    let mut result = vec![];
    for (index, value) in values.iter().enumerate() {
        for other_value in values.iter().skip(index + 1) {
            result.push(((*value).clone(), (*other_value).clone()));
        }
    }
    result
}

fn find_circuit(circuits: &[HashSet<JunctionBox>], junction_box: &JunctionBox) -> Option<usize> {
    for (index, circuit) in circuits.iter().enumerate() {
        if circuit.contains(junction_box) {
            return Some(index);
        }
    }

    None
}

fn largest_circuits(junction_boxes: &[JunctionBox], limit: usize) -> Vec<HashSet<JunctionBox>> {
    let closest_pairs: Vec<(JunctionBox, JunctionBox)> = unique_pairs(junction_boxes)
        .into_iter()
        .sorted_by_key(|pair| pair.0.euclidian_distance(&pair.1))
        .take(1000)
        .collect();

    let mut circuits = Vec::<HashSet<JunctionBox>>::new();

    for pair in &closest_pairs {
        let (left_box, right_box) = pair;
        match (
            find_circuit(&circuits, left_box),
            find_circuit(&circuits, right_box),
        ) {
            (None, None) => {
                let new_circuit = HashSet::from([left_box.clone(), right_box.clone()]);
                circuits.push(new_circuit);
            }
            (None, Some(right_circuit_index)) => {
                circuits[right_circuit_index].insert(left_box.clone());
            }
            (Some(left_circuit_index), None) => {
                circuits[left_circuit_index].insert(right_box.clone());
            }
            (Some(left_circuit_index), Some(right_circuit_index)) => {
                if left_circuit_index == right_circuit_index {
                    continue;
                }
                let right_circuits: Vec<JunctionBox> =
                    circuits[right_circuit_index].drain().collect();
                for circuit in right_circuits {
                    circuits[left_circuit_index].insert(circuit);
                }
            }
        }
    }

    circuits
        .into_iter()
        .sorted_by_key(|circuit| circuit.len())
        .rev()
        .take(limit)
        .collect()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct JunctionBox {
    x: usize,
    y: usize,
    z: usize,
}

impl JunctionBox {
    fn euclidian_distance(&self, other: &JunctionBox) -> isize {
        ((other.x as isize - self.x as isize).pow(2)
            + (other.y as isize - self.y as isize).pow(2)
            + (other.z as isize - self.z as isize).pow(2))
        .isqrt()
    }
}

fn parse(path: &str) -> Vec<JunctionBox> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            let values: Vec<usize> = line
                .split(",")
                .map(|value| value.parse().unwrap())
                .collect();

            JunctionBox {
                x: values[0],
                y: values[1],
                z: values[2],
            }
        })
        .collect()
}
