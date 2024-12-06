use crate::Direction::{East, North, South, West};
use rayon::iter::ParallelIterator;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator};
use std::collections::BTreeSet;

/*

    Part 1 Approach:

    Read input into graph:
    . = open node
    # = obstacle

    Search:
    Always turn right when an obstacle is encountered.
    Mark visited nodes with a +
    Maintain a count of unique visited nodes (not +) include the starting node.

    Exit condition: next node is out of bounds.
    Return count.

    Part 2 Approach:

    Try inserting an obstacle.
    Test for a cycle using DFS.
    Return number of cycles.

*/

fn main() {
    let input = include_str!("../input");

    let (mat, start, start_direction) = read_input(input).unwrap();
    let count = get_count(1, start, start_direction, &mut mat.clone());

    println!("Distinct positions visited: {}", count);

    let (mat, start, start_direction) = read_input(input).unwrap();
    let cycles = get_cycles(&mat, start, start_direction);

    println!("Cycles detected: {}", cycles);
}

#[derive(Copy, Clone, Ord, PartialOrd, PartialEq, Eq)]
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

fn read_input(input: &str) -> Result<(Vec<Vec<char>>, (usize, usize), Direction), InputReadError> {
    let rows = input.lines().count();
    let cols = input.lines().next().iter().count();

    let mut start = None;
    let mut start_direction = None;

    let mut matrix: Vec<Vec<char>> = Vec::with_capacity(rows);

    input.lines().enumerate().for_each(|(row, line)| {
        matrix.push(Vec::with_capacity(cols));
        line.split("")
            .filter_map(|s| s.parse::<char>().ok())
            .enumerate()
            .for_each(|(col, c)| {
                matrix
                    .get_mut(row)
                    .expect(format!("Row {} not inserted", row).as_str())
                    .push(c);

                if let Ok(dir) = Direction::try_from(c) {
                    start_direction = Some(dir);
                    start = Some((row, col));
                }
            });
    });

    if let Some(start) = start {
        if let Some(start_direction) = start_direction {
            return Ok((matrix, start, start_direction));
        }
    }

    Err(InputReadError::NoStart)
}

#[derive(Debug)]
enum NodeError {
    OutOfBounds,
}

fn get_node(at: (usize, usize), matrix: &Vec<Vec<char>>) -> Result<char, NodeError> {
    let (r, c) = at;
    let row = matrix.get(r).ok_or(NodeError::OutOfBounds)?;
    let char = row.get(c).ok_or(NodeError::OutOfBounds)?;

    Ok(*char)
}

fn get_count(
    mut count: i32,
    start: (usize, usize),
    dir: Direction,
    mat: &mut Vec<Vec<char>>,
) -> i32 {
    let Ok(c) = get_node(start, &mat) else {
        return count;
    };

    if c == '.' {
        count += 1;
    }

    let (row, col) = start;
    if let Some(row) = mat.get_mut(row) {
        if let Some(c) = row.get_mut(col) {
            *c = '+';
        }
    }

    let Some((nr, nc)) = get_next_in_dir(&start, dir) else {
        return count;
    };

    let Ok(next_c) = get_node((nr, nc), mat) else {
        return count;
    };

    let dir = if next_c == '#' { dir.turn() } else { dir };

    let Some((nr, nc)) = get_next_in_dir(&start, dir) else {
        return count;
    };

    get_count(count, (nr, nc), dir, mat)
}

fn get_next_in_dir(start: &(usize, usize), dir: Direction) -> Option<(usize, usize)> {
    let (row, col) = *start;
    let (dr, dc) = dir.get_vec();
    let (nr, nc) = (row as i32 + dr, col as i32 + dc);

    if nr < 0 || nc < 0 {
        return None;
    }

    Some((nr as usize, nc as usize))
}

fn get_cycles(mat: &Vec<Vec<char>>, start: (usize, usize), direction: Direction) -> i32 {
    let count: i32 = mat
        .par_iter()
        .enumerate()
        .map(|(row_n, row)| {
            let add: i32 = row
                .par_iter()
                .enumerate()
                .map(|(col_n, c)| {
                    if *c != '#' && Direction::try_from(*c).is_err() {
                        let block = (row_n, col_n);
                        return if find_cycle(
                            &mat.clone(),
                            block,
                            start,
                            direction,
                            &mut BTreeSet::new(),
                        ) {
                            1
                        } else {
                            0
                        };
                    }
                    0
                })
                .sum();
            add
        })
        .sum();

    count
}

fn find_cycle(
    mat: &Vec<Vec<char>>,
    blocked: (usize, usize),
    start: (usize, usize),
    direction: Direction,
    nodes: &mut BTreeSet<((usize, usize), Direction)>,
) -> bool {
    let mut start = start;
    let mut direction = direction;
    while let Some((node, new_dir)) = get_next_node_2(mat, blocked, start, direction) {
        if nodes.contains(&(node, new_dir)) {
            return true;
        } else {
            nodes.insert((node, new_dir));
            direction = new_dir;
            start = node;
        }
    }

    false
}

fn get_next_node_2(
    mat: &Vec<Vec<char>>,
    blocked: (usize, usize),
    start: (usize, usize),
    dir: Direction,
) -> Option<((usize, usize), Direction)> {
    let Some((nr, nc)) = get_next_in_dir(&start, dir) else {
        return None;
    };

    let Ok(c) = get_node((nr, nc), mat) else {
        return None;
    };

    let new_dir = if (nr, nc) == blocked || c == '#' {
        return Some((start, dir.turn()));
    } else {
        dir
    };

    get_next_node_2(mat, blocked, (nr, nc), new_dir)
}
