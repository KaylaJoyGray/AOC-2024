use crate::Direction::{East, North, South, West};
use std::collections::BTreeMap;

/*  Part 1 Approach:

    Read input into graph:
    . = open node
    # = obstacle

    Search:
    Always turn right when an obstacle is encountered.
    Mark visited nodes with a +
    Maintain a count of unique visited nodes (not +) include the starting node.

    Exit condition: next node is out of bounds.
    Return count.

*/

fn main() {
    let input = include_str!("../input");

    let (mut map, start, start_direction) = read_input(input).unwrap();
    let count = get_count(1, start, start_direction, &mut map);

    println!("Distinct positions visited: {}", count);
}

const DIRECTIONS: [char; 4] = ['^', 'v', '>', '<'];

enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
enum ParseDirectionError {
    InvalidChar,
}

impl TryFrom<char> for Direction {
    type Error = ParseDirectionError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(North),
            'v' => Ok(South),
            '>' => Ok(East),
            '<' => Ok(West),
            _ => Err(ParseDirectionError::InvalidChar),
        }
    }
}

impl Direction {
    fn get_vec(&self) -> (i32, i32) {
        match self {
            North => (-1, 0),
            South => (1, 0),
            East => (0, 1),
            West => (0, -1),
        }
    }

    fn turn(&self) -> Direction {
        match self {
            North => East,
            South => West,
            East => South,
            West => North,
        }
    }
}

#[derive(Debug)]
enum InputReadError {
    NoStart,
}

fn read_input(
    input: &str,
) -> Result<(BTreeMap<(u32, u32), char>, (u32, u32), Direction), InputReadError> {
    let mut map = BTreeMap::new();
    let mut start = None;
    let mut start_direction = None;

    input.lines().enumerate().for_each(|(row, line)| {
        line.split("")
            .filter_map(|s| s.parse::<char>().ok())
            .enumerate()
            .for_each(|(col, c)| {
                map.entry((row as u32, col as u32)).or_insert(c);
                if DIRECTIONS.contains(&c) {
                    start = Some((row as u32, col as u32));
                    start_direction = Direction::try_from(c).ok();
                }
            })
    });

    if let Some(start) = start {
        if let Some(start_direction) = start_direction {
            return Ok((map, start, start_direction));
        }
    }

    Err(InputReadError::NoStart)
}

fn get_next_dir(start: &(u32, u32), dir: &Direction) -> Option<(u32, u32)> {
    let (row, col) = *start;
    let (dr, dc) = dir.get_vec();
    let (nr, nc) = (row as i32 + dr, col as i32 + dc);

    if nr < 0 || nc < 0 {
        return None;
    }

    Some((nr as u32, nc as u32))
}

fn get_count(
    mut count: i32,
    start: (u32, u32),
    dir: Direction,
    map: &mut BTreeMap<(u32, u32), char>,
) -> i32 {
    let Some(c) = map.get_mut(&start) else {
        return count;
    };

    if *c == '.' {
        count += 1;
    }

    *c = '+';

    let Some((nr, nc)) = get_next_dir(&start, &dir) else {
        return count;
    };

    let Some(next_c) = map.get(&(nr, nc)) else {
        return count;
    };

    let dir = if *next_c == '#' { dir.turn() } else { dir };

    let Some((nr, nc)) = get_next_dir(&start, &dir) else {
        return count;
    };

    get_count(count, (nr, nc), dir, map)
}
