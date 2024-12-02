use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    let vec = read_input();

    println!("Safe Reports: {}", calc_safe(&vec));
    println!(
        "Safe after problem dampener: {}",
        calc_safe_with_dampener(&vec)
    );
}

fn read_input() -> Vec<Vec<i32>> {
    let file = fs::File::open("input").expect("Could not read input");

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

    vec
}

fn calc_safe(vec: &Vec<Vec<i32>>) -> i32 {
    vec.iter()
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
        .sum()
}

fn calc_safe_with_dampener(vec: &Vec<Vec<i32>>) -> i32 {
    vec.iter()
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
        .sum()
}
