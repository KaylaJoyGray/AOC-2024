use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    // Each report is a list of numbers called levels that are separated by spaces. For example:
    //
    //  7 6 4 2 1
    //  1 2 7 8 9
    //  9 7 6 2 1
    //  1 3 2 4 5
    //  8 6 4 4 1
    //  1 3 6 7 9
    //
    // This example data contains six reports each containing five levels.
    //
    // The report only counts as safe if both of the following are true:
    //
    //     - The levels are either all increasing or all decreasing.
    //     - Any two adjacent levels differ by at least one and at most three.

    let Ok(file) = fs::File::open("input") else {
        println!("Could not read input!");
        return;
    };

    let mut vec: Vec<Vec<i32>> = Vec::new();
    BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| {
            let line = line
                .split(' ')
                .filter_map(|s| s.parse::<i32>().ok())
                .collect::<Vec<i32>>();

            vec.push(line);
        });

    let safe: i32 = vec
        .iter()
        .filter_map(|row| {
            let mut ascending = false;
            let mut descending = false;

            let mut peekable = row.iter().peekable();
            while let Some(n) = peekable.next() {
                if let Some(next) = peekable.peek() {
                    if n < *next {
                        if descending {
                            return None;
                        }
                        ascending = true;
                    } else if n > *next {
                        if ascending {
                            return None;
                        }
                        descending = true;
                    } else {
                        return None;
                    }

                    let diff = n.abs_diff(**next);

                    if diff < 1 {
                        return None;
                    } else if diff > 3 {
                        return None;
                    }
                }
            }

            Some(1)
        })
        .sum();

    println!("Safe Reports: {}", safe);

    // safety systems tolerate a single bad level in what would otherwise be a safe report.
    // Now, the same rules apply as before, except if removing a single level from an unsafe
    // report would make it safe, the report instead counts as safe.

    let safe: i32 = vec
        .iter()
        .filter_map(|row| {
            let mut remove = false;
            let mut ascending = false;
            let mut descending = false;

            let mut check_remove = || {
                if remove == false {
                    remove = true;
                    true
                } else {
                    false
                }
            };

            let mut peekable = row.iter().peekable();
            while let Some(n) = peekable.next() {
                if let Some(next) = peekable.peek() {
                    if n < *next {
                        if descending {
                            if check_remove() {
                                continue;
                            } else {
                                return None;
                            }
                        }
                        ascending = true;
                    } else if n > *next {
                        if ascending {
                            if check_remove() {
                                continue;
                            } else {
                                return None;
                            }
                        }
                        descending = true;
                    } else {
                        if check_remove() {
                            continue;
                        } else {
                            return None;
                        }
                    }

                    let diff = n.abs_diff(**next);

                    if diff < 1 {
                        if check_remove() {
                            continue;
                        } else {
                            return None;
                        }
                    } else if diff > 3 {
                        if check_remove() {
                            continue;
                        } else {
                            return None;
                        }
                    }
                }
            }

            Some(1)
        })
        .sum();

    println!("Safe after problem dampener: {}", safe);
}
