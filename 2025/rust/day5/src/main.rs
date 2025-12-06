use std::{fs, ops::RangeInclusive};

fn main() {
    let (db, ingredients) = parse_input("src/input");
    let fresh_ingredients = ingredients
        .iter()
        .filter(|ingredient| db.contains(ingredient))
        .count();

    dbg!(fresh_ingredients); // 529

    let all_fresh_ingredients = db.count();
    dbg!(all_fresh_ingredients); // 344260049617193
}

type IngredientId = u64;
type Ingredients = Vec<IngredientId>;

struct FreshDb(Vec<RangeInclusive<IngredientId>>);

impl FreshDb {
    pub fn new(ranges: Vec<RangeInclusive<IngredientId>>) -> FreshDb {
        let mut _ranges = ranges;
        _ranges.sort_by_key(|range| *range.start());
        FreshDb(_ranges)
    }
    pub fn contains(&self, ingredient_id: &IngredientId) -> bool {
        self.0.iter().any(|range| range.contains(ingredient_id))
    }

    pub fn count(&self) -> u64 {
        let mut merged_ranges = vec![self.0[0].clone()];
        let mut last = self.0[0].clone();
        for range in &self.0 {
            if range.start() > last.end() {
                last = *range.start()..=*range.end();
                merged_ranges.push(last.clone());
            } else if range.end() >= last.end() {
                merged_ranges.pop().unwrap();
                last = *last.start()..=*range.end();
                merged_ranges.push(last.clone());
            }
        }

        merged_ranges
            .iter()
            .map(|range| range.end() - range.start() + 1)
            .sum()
    }
}

fn parse_input(path: &str) -> (FreshDb, Ingredients) {
    let parts: Vec<String> = fs::read_to_string(path)
        .unwrap()
        .split("\n\n")
        .map(|line| line.to_owned())
        .collect();

    let ranges: Vec<RangeInclusive<IngredientId>> = parts[0]
        .lines()
        .map(|line| {
            let range: Ingredients = line.split("-").map(|part| part.parse().unwrap()).collect();
            range[0]..=range[1]
        })
        .collect();

    let ingredients: Ingredients = parts[1].lines().map(|line| line.parse().unwrap()).collect();
    (FreshDb::new(ranges), ingredients)
}
