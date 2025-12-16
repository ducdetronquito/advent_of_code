use std::{fs, vec};

fn main() {
    let part1_total: u64 = parse_problems("./src/input", read_numbers_horizontally)
        .iter()
        .map(|problem| problem.solve())
        .sum();
    dbg!(part1_total); // 5977759036837

    let part2_total: u64 = parse_problems("./src/input", read_numbers_vertically)
        .iter()
        .map(|problem| problem.solve())
        .sum();

    dbg!(part2_total); // 9630000828442
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug)]
struct Problem {
    numbers: Vec<u64>,
    operator: Operator,
}

impl Problem {
    fn solve(&self) -> u64 {
        match self.operator {
            Operator::Add => self.numbers.iter().sum(),
            Operator::Multiply => self.numbers.iter().product(),
        }
    }
}

fn parse_number(value: &[u8]) -> u64 {
    String::from_utf8(value.to_vec())
        .unwrap()
        .replace(" ", "")
        .parse()
        .unwrap()
}

fn read_numbers_horizontally(
    number_lines: &[String],
    column_start: usize,
    column_end: usize,
) -> Vec<u64> {
    number_lines
        .iter()
        .map(|number_line| parse_number(&number_line.as_bytes()[column_start..column_end]))
        .collect()
}

fn read_numbers_vertically(
    number_lines: &[String],
    column_start: usize,
    column_end: usize,
) -> Vec<u64> {
    let mut numbers = vec![];

    for column_index in column_start..column_end {
        let raw_number: Vec<u8> = number_lines
            .iter()
            .map(|number_line| number_line.as_bytes()[column_index])
            .collect();

        numbers.push(parse_number(&raw_number))
    }

    numbers
}

fn parse_problems(
    path: &str,
    parse_numbers: fn(&[String], usize, usize) -> Vec<u64>,
) -> Vec<Problem> {
    let input: String = fs::read_to_string(path).unwrap();
    let lines: Vec<String> = input.lines().map(|line| line.to_owned()).collect();

    let number_lines = &lines[..lines.len() - 1];
    let raw_operators: Vec<char> = lines[lines.len() - 1].chars().collect();

    let mut problems = vec![];
    for (index, item) in raw_operators.iter().enumerate() {
        if *item == ' ' {
            continue;
        }
        let operator = if *item == '+' {
            Operator::Add
        } else {
            Operator::Multiply
        };

        let problem_start = index;
        let problem_end = raw_operators
            .iter()
            .skip(problem_start + 1)
            .position(|item| *item == '+' || *item == '*')
            .map(|next_operator_position| problem_start + next_operator_position)
            .unwrap_or(problem_start + raw_operators.iter().skip(problem_start).len());

        problems.push(Problem {
            operator,
            numbers: parse_numbers(number_lines, problem_start, problem_end),
        });
    }

    problems
}
