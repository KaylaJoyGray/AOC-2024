use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Part 1 Approach:
    // Populate a lookup table. The right part of the rule will map to all matching left parts.
    // I.E entry 53 will contain all page numbers that must be printed before page 53.
    //
    // Iterate through each update, and maintain a list of visited numbers.
    // If any part of the entry for the current number is in visited, then the update is invalid.
    // If the end is reached without triggering the above condition, then the update is valid.
    //

    // Part 2 Approach:
    // Modified the function from part 1 to collect and return all invalid updates.
    // Sorted with a custom comparison function using the lookup table from part 1.
    //

    let Ok(file) = File::open("input") else {
        panic!("Could not open input file")
    };

    let mut lookup: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    parse_input(&file, &mut lookup, &mut updates);
    let (sum, mut invalid_updates) = get_sum(&lookup, &updates);

    println!("Sum of middle pages: {}", sum);

    let sum_2 = sort_and_get_sum(&lookup, &mut invalid_updates);

    println!("Sum of sorted middle pages: {}", sum_2);
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
                .entry(left)
                .and_modify(|entry| entry.push(right))
                .or_insert(vec![right]);
        }
    });
}

fn get_sum(lookup: &BTreeMap<i32, Vec<i32>>, updates: &Vec<Vec<i32>>) -> (i32, Vec<Vec<i32>>) {
    let mut invalid: Vec<Vec<i32>> = Vec::new();

    let sum: i32 = updates
        .iter()
        .filter_map(|update| {
            let mut previous: Vec<i32> = Vec::with_capacity(update.len());
            let count = update
                .iter()
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
        .sum();

    (sum, invalid)
}

fn sort_and_get_sum(lookup: &BTreeMap<i32, Vec<i32>>, updates: &mut Vec<Vec<i32>>) -> i32 {
    updates.iter_mut().for_each(|update| {
        update.sort_by(|a, b| {
            if let Some(matches) = lookup.get(a) {
                if matches.contains(b) {
                    Less
                } else {
                    Greater
                }
            } else {
                Equal
            }
        });
    });

    updates
        .iter()
        .filter_map(|update| {
            let middle = update.len() / 2;
            update.get(middle)
        })
        .sum::<i32>()
}
