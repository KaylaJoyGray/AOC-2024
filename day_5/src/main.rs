use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Part 1 Approach:
    // Populate a lookup table. The right part of the rule will map to all matching left parts.
    // So entry 53 will contain all numbers that must be printed before 53
    //
    // Iterate through each update in reverse order, and maintain a visited list of visited numbers.
    // If any part of the entry for the current number is in visited, then the update is invalid.
    // If the front is reached without triggering the above condition, then the update is valid.
    //

    let Ok(file) = File::open("input") else {
        panic!("Could not open input file")
    };

    let mut lookup: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    parse_input(&file, &mut lookup, &mut updates);
    let (sum, invalid_updates) = get_sum(&lookup, &updates);

    println!("Sum of middle pages: {}", sum);
}

fn parse_input(input: &File, lookup: &mut BTreeMap<i32, Vec<i32>>, updates: &mut Vec<Vec<i32>>) {
    let reader = BufReader::new(input);

    reader.lines().map_while(|line| line.ok()).for_each(|line| {
        if line.contains(',') {
            let update = line
                .split(',')
                .filter_map(|s| s.parse::<i32>().ok())
                .collect::<Vec<i32>>();
            updates.push(update);
        } else if line.contains('|') {
            let mut split = line.split('|').filter_map(|s| s.parse::<i32>().ok());
            let left = split.next().expect("Invalid rule encountered");
            let right = split.last().expect("Invalid rule encountered");
            lookup
                .entry(right)
                .and_modify(|entry| entry.push(left))
                .or_insert(vec![left]);
        }
    });
}

fn get_sum(lookup: &BTreeMap<i32, Vec<i32>>, updates: &Vec<Vec<i32>>) -> (i32, Vec<Vec<i32>>) {
    let mut invalid: Vec<Vec<i32>> = Vec::new();

    let sum: i32 = updates
        .iter()
        .filter_map(|update| {
            let mut previous: Vec<i32> = Vec::new();
            let count = update
                .iter()
                .rev()
                .take_while(|n| {
                    if let Some(matches) = lookup.get(n) {
                        for p in &previous {
                            if matches.contains(p) {
                                return false;
                            }
                        }
                    }
                    previous.push(**n);
                    true
                })
                .count();

            if count == update.len() {
                let middle = update.len() / 2;
                update.get(middle)
            } else {
                invalid.push(update.clone());
                None
            }
        })
        .map(|n| *n)
        .collect::<Vec<i32>>()
        .iter()
        .sum();

    (sum, invalid)
}
