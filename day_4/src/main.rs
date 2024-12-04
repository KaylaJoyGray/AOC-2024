use std::collections::BTreeMap;

fn main() {
    let input = include_str!("../input").trim();

    let (rows, cols) = get_dims(input);

    let map = read_into_map(input, cols);

    let count = word_count(&map, rows, cols);

    println!("{}", count);
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

const REL_NEIGHBORS: [(i32, i32); 8] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
    (1, -1),
    (-1, 1),
    (1, 1),
    (-1, -1),
];

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

    *previous += &current.to_string();

    if previous == "XMAS" {
        return 1;
    }

    if previous != "X" && previous != "XM" && previous != "XMA" {
        return 0;
    }

    let (dr, dc) = direction;
    let (nr, nc) = (row as i32 + dr, col as i32 + dc);

    if nr < 0 || nc < 0 {
        return 0;
    }

    find_word(nr as u32, nc as u32, direction, map, previous)
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
