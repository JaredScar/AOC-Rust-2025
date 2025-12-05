use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/day05.txt").expect("Failed to read input file");
    let mut p1 = 0;
    let mut p2 = 0;
    let mut ingredient_ranges = vec![];
    let mut ingredient_ids = vec![];
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line.contains("-") {
            let low_end = line.split("-").next().unwrap().parse::<i64>().unwrap();
            let high_end = line.split("-").nth(1).unwrap().parse::<i64>().unwrap();
            ingredient_ranges.push((low_end, high_end));
        } else {
            ingredient_ids.push(line.parse::<i64>().unwrap());
        }
    }

    // Merge overlapping ranges for part 2
    ingredient_ranges.sort_by_key(|r| r.0);
    let mut merged_ranges: Vec<(i64, i64)> = vec![];
    for range in ingredient_ranges {
        if let Some(last) = merged_ranges.last_mut() {
            if range.0 <= last.1 + 1 {
                // Overlapping or adjacent ranges - merge them
                last.1 = last.1.max(range.1);
            } else {
                merged_ranges.push(range);
            }
        } else {
            merged_ranges.push(range);
        }
    }
    ingredient_ranges = merged_ranges;

    for ingredient_id in ingredient_ids {
        let mut is_fresh = false;
        for range in &ingredient_ranges {
            if ingredient_id >= range.0 && ingredient_id <= range.1 {
                is_fresh = true;
                break;
            }
        }
        if is_fresh {
            p1 += 1;
        }
    }
    for range in ingredient_ranges {
        p2 += range.1 - range.0 + 1;
    }
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}