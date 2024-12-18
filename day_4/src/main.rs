use std::collections::BTreeMap;

fn main() {
    let input = include_str!("../input").trim();

    let (rows, cols) = get_dims(input);

    let map = read_into_map(input, cols);

    let count = word_count(&map, rows, cols);

    println!("XMAS count: {}", count);

    let count2 = word_count_2(&map, rows, cols);

    println!("MAS crossed count: {}", count2);
}

fn get_dims(input: &str) -> (u32, u32) {
    (
        input.chars().position(|c| c == '\n').unwrap() as u32,
        input.lines().count() as u32,
    )
}

fn read_into_map(input: &str, cols: u32) -> BTreeMap<(u32, u32), char> {
    let mut chars: BTreeMap<(u32, u32), char> = BTreeMap::new();

    for (index, c) in input
        .chars()
        .filter_map(|c| if c == '\n' { None } else { Some(c) })
        .enumerate()
    {
        let index = index as u32;
        chars.entry((index / cols, index % cols)).or_insert(c);
    }

    chars
}

const REL_NEIGHBORS: [(i32, i32); 4] = [(0, 1), (1, 0), (1, -1), (1, 1)];

fn word_count(map: &BTreeMap<(u32, u32), char>, rows: u32, cols: u32) -> i32 {
    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            count += REL_NEIGHBORS
                .iter()
                .map(|(dr, dc)| find_word(row, col, &(*dr, *dc), &map, &mut "".to_string()))
                .sum::<i32>();
        }
    }

    count
}

const SOLVED_XMAS: [&str; 2] = ["XMAS", "SAMX"];
const VALID_XMAS_SEQ: [&str; 6] = ["X", "XM", "XMA", "S", "SA", "SAM"];

fn find_word(
    row: u32,
    col: u32,
    direction: &(i32, i32),
    map: &BTreeMap<(u32, u32), char>,
    previous: &mut String,
) -> i32 {
    let Some(current) = map.get(&(row, col)) else {
        return 0;
    };

    previous.push(*current);

    if SOLVED_XMAS.contains(&previous.as_ref()) {
        return 1;
    }

    if !VALID_XMAS_SEQ.contains(&previous.as_ref()) {
        return 0;
    }

    let (dr, dc) = direction;
    let (nr, nc) = (row as i32 + dr, col as i32 + dc);

    if nr < 0 || nc < 0 {
        return 0;
    }

    find_word(nr as u32, nc as u32, direction, map, previous)
}

const DIR_1: (i32, i32) = (1, 1);
const DIR_2: (i32, i32) = (1, -1);

fn word_count_2(map: &BTreeMap<(u32, u32), char>, rows: u32, cols: u32) -> i32 {
    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            let (r1, c1) = DIR_1;
            let (r2, c2) = DIR_2;

            if find_word_2(row, col, &(r1, c1), &map, &mut "".to_string()) {
                if find_word_2(row, col + 2, &(r2, c2), &map, &mut "".to_string()) {
                    count += 1;
                }
            }
        }
    }

    count
}

const SOLVED_MAS: [&str; 2] = ["MAS", "SAM"];
const VALID_MAS_SEQ: [&str; 4] = ["M", "MA", "S", "SA"];

fn find_word_2(
    row: u32,
    col: u32,
    direction: &(i32, i32),
    map: &BTreeMap<(u32, u32), char>,
    previous: &mut String,
) -> bool {
    let Some(current) = map.get(&(row, col)) else {
        return false;
    };

    previous.push(*current);

    if SOLVED_MAS.contains(&previous.as_ref()) {
        return true;
    }

    if !VALID_MAS_SEQ.contains(&previous.as_ref()) {
        return false;
    }

    let (dr, dc) = direction;
    let (nr, nc) = (row as i32 + dr, col as i32 + dc);

    if nr < 0 || nc < 0 {
        return false;
    }

    find_word_2(nr as u32, nc as u32, direction, map, previous)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_dims() {
        assert_eq!(get_dims(&"000\n000\n000"), (3, 3));
        assert_eq!(get_dims(&"0000\n0000\n0000\n0000"), (4, 4));
    }

    #[test]
    fn test_read_into_map() {
        let str = "162\n789\n304";
        let map = read_into_map(str, 3);
        assert_eq!(*map.get(&(0, 0)).unwrap(), '1');
        assert_eq!(*map.get(&(2, 0)).unwrap(), '3');
        assert_eq!(*map.get(&(0, 2)).unwrap(), '2');
        assert_eq!(*map.get(&(2, 2)).unwrap(), '4');
    }
}
