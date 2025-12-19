use std::{collections::HashSet, fs};

fn main() {
    let diagram = parse_diagram("./src/input");

    dbg!(count_beam_splits(&diagram)); // 1649

    dbg!(count_timelines(&diagram)); // Works on test input but do not finish on the actual input
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Beam {
    x: usize,
    y: usize,
    state: BeamState,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum BeamState {
    EnterDiagram,
    ExitedDiagram,
    Extended,
    HitSplitter,
}

impl Beam {
    fn move_downward(&self, diagram: &Diagram) -> Beam {
        let next_x = self.x;
        let next_y = self.y + 1;
        let state = if next_x >= diagram.values[0].len() || next_y >= diagram.values.len() {
            BeamState::ExitedDiagram
        } else if diagram.values[next_y][next_x] == '^' {
            BeamState::HitSplitter
        } else {
            BeamState::Extended
        };
        Beam {
            x: next_x,
            y: next_y,
            state,
        }
    }

    fn split(&self) -> (Beam, Beam) {
        (
            Beam {
                x: self.x - 1,
                y: self.y,
                state: BeamState::HitSplitter,
            },
            Beam {
                x: self.x + 1,
                y: self.y,
                state: BeamState::HitSplitter,
            },
        )
    }
}

#[derive(Debug)]
struct Diagram {
    values: Vec<Vec<char>>,
    beam_entry: Beam,
}

#[derive(Debug)]
struct Step {
    beams: HashSet<Beam>,
    split_count: usize,
}

fn count_beam_splits(diagram: &Diagram) -> usize {
    let mut current_step = Step {
        beams: HashSet::from([diagram.beam_entry.clone()]),
        split_count: 0,
    };

    while !current_step.beams.is_empty() {
        current_step = propagate_beam(diagram, current_step);
    }

    current_step.split_count
}

fn propagate_beam(diagram: &Diagram, step: Step) -> Step {
    let mut next_beams = HashSet::new();
    let mut split_count = step.split_count;
    for beam in &step.beams {
        let next_beam = beam.move_downward(diagram);
        match next_beam.state {
            BeamState::Extended => {
                next_beams.insert(next_beam);
            }
            BeamState::HitSplitter => {
                let (left_beam, right_beam) = next_beam.split();
                next_beams.insert(left_beam);
                next_beams.insert(right_beam);
                split_count += 1;
            }
            BeamState::EnterDiagram | BeamState::ExitedDiagram => {}
        }
    }

    Step {
        beams: next_beams,
        split_count,
    }
}

fn parse_diagram(path: &str) -> Diagram {
    let values: Vec<Vec<char>> = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut beam_entry = Beam {
        x: 0,
        y: 0,
        state: BeamState::EnterDiagram,
    };
    for (y, row) in values.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == 'S' {
                beam_entry.x = x;
                beam_entry.y = y;
            }
        }
    }

    Diagram { values, beam_entry }
}

fn count_timelines_rec(diagram: &Diagram, beam: &Beam, counter: usize) -> usize {
    let next_beam = beam.move_downward(diagram);

    match next_beam.state {
        BeamState::EnterDiagram => count_timelines_rec(diagram, &next_beam, counter),
        BeamState::Extended => count_timelines_rec(diagram, &next_beam, counter),
        BeamState::HitSplitter => {
            let (left_beam, right_beam) = next_beam.split();

            count_timelines_rec(diagram, &left_beam, counter)
                + count_timelines_rec(diagram, &right_beam, counter)
        }
        BeamState::ExitedDiagram => counter + 1,
    }
}

fn count_timelines(diagram: &Diagram) -> usize {
    count_timelines_rec(diagram, &diagram.beam_entry, 0)
}

#[test]
fn test_move_downward() {
    let diagram = parse_diagram("./src/test-input");

    assert_eq!(
        Beam {
            x: 0,
            y: 0,
            state: BeamState::EnterDiagram
        }
        .move_downward(&diagram)
        .state,
        BeamState::Extended
    );
    assert_eq!(
        Beam {
            x: 0,
            y: 15,
            state: BeamState::EnterDiagram
        }
        .move_downward(&diagram)
        .state,
        BeamState::ExitedDiagram
    );
    assert_eq!(
        Beam {
            x: 0,
            y: 16,
            state: BeamState::EnterDiagram
        }
        .move_downward(&diagram)
        .state,
        BeamState::ExitedDiagram
    );
    assert_eq!(
        Beam {
            x: 14,
            y: 0,
            state: BeamState::EnterDiagram
        }
        .move_downward(&diagram)
        .state,
        BeamState::Extended
    );
    assert_eq!(
        Beam {
            x: 15,
            y: 0,
            state: BeamState::EnterDiagram
        }
        .move_downward(&diagram)
        .state,
        BeamState::ExitedDiagram
    );
    assert_eq!(
        Beam {
            x: 14,
            y: 15,
            state: BeamState::EnterDiagram
        }
        .move_downward(&diagram)
        .state,
        BeamState::ExitedDiagram
    );
    assert_eq!(
        Beam {
            x: 7,
            y: 1,
            state: BeamState::Extended
        }
        .move_downward(&diagram)
        .state,
        BeamState::HitSplitter
    );
}

#[test]
fn test_count_timelines_very_small_input() {
    let diagram = parse_diagram("./src/very-small-input");
    assert_eq!(count_timelines(&diagram), 2)
}

#[test]
fn test_count_timelines_small_input() {
    let diagram = parse_diagram("./src/small-input");
    assert_eq!(count_timelines(&diagram), 4)
}

#[test]
fn test_count_timelines_test_input() {
    let diagram = parse_diagram("./src/test-input");
    assert_eq!(count_timelines(&diagram), 40)
}
