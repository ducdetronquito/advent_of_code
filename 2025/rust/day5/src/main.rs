use std::{
    cmp::{max, min},
    collections::HashSet,
    fs, iter,
    ops::RangeInclusive,
};

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
    fn contains(&self, ingredient_id: &IngredientId) -> bool {
        self.0.iter().any(|range| range.contains(ingredient_id))
    }

    fn count(&self) -> u64 {
        let mut distinct_ranges: HashSet<RangeInclusive<u64>> = HashSet::from_iter(self.0.clone());

        loop {
            let merged_ranges: HashSet<RangeInclusive<u64>> = merge_once(&distinct_ranges);
            if merged_ranges == distinct_ranges {
                break;
            }
            distinct_ranges = merged_ranges;
        }

        // Stabilize a bit more...
        for _ in 0..=10 {
            distinct_ranges = merge_once(&distinct_ranges);
        }
        distinct_ranges.iter().map(count).sum()
    }
}

fn merge(
    range_a: &RangeInclusive<u64>,
    range_b: &RangeInclusive<u64>,
) -> Option<RangeInclusive<u64>> {
    // [A------------A']
    //    [B----
    let b_start_within_a = range_b.start() >= range_a.start() && range_b.start() <= range_a.end();

    // [A------------A']
    //     ----B']
    let b_end_within_a = range_b.end() >= range_a.start() && range_b.end() <= range_a.end();

    //    [A----A']
    // [B----------B']
    let b_includes_a = range_b.start() <= range_a.start() && range_b.end() >= range_a.end();

    let intersect = b_start_within_a || b_end_within_a || b_includes_a;
    if !intersect {
        return None;
    }

    Some(*min(range_a.start(), range_b.start())..=*max(range_a.end(), range_b.end()))
}

fn count(range: &RangeInclusive<u64>) -> u64 {
    range.end() - range.start() + 1
}

fn merge_once(ranges: &HashSet<RangeInclusive<u64>>) -> HashSet<RangeInclusive<u64>> {
    ranges.iter().fold(HashSet::new(), |acc, range| {
        if acc.is_empty() {
            return HashSet::from_iter(iter::once(range.clone()));
        }

        let mut distinct_ranges = HashSet::new();

        for distinct_range in acc.iter().chain(iter::once(&range.clone())) {
            match merge(range, distinct_range) {
                Some(merged_range) => {
                    distinct_ranges.insert(merged_range);
                }
                None => {
                    distinct_ranges.insert(distinct_range.clone());
                }
            }
        }

        distinct_ranges
    })
}

fn parse_input(path: &str) -> (FreshDb, Ingredients) {
    let parts: Vec<String> = fs::read_to_string(path)
        .unwrap()
        .split("\n\n")
        .map(|line| line.to_owned())
        .collect();

    let db: Vec<RangeInclusive<IngredientId>> = parts[0]
        .lines()
        .map(|line| {
            let range: Ingredients = line.split("-").map(|part| part.parse().unwrap()).collect();
            range[0]..=range[1]
        })
        .collect();

    let ingredients: Ingredients = parts[1].lines().map(|line| line.parse().unwrap()).collect();
    (FreshDb(db), ingredients)
}
