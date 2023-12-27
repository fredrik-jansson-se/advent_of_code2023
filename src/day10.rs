use std::{collections::HashSet, usize};

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day10.txt")?;

    println!("10:1 - {}", run_1(&input)?);
    println!("10:2 - {}", run_2(&input)?);

    Ok(())
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let map: Map = input.lines().map(|row| row.chars().collect()).collect();

    let start: Coord = map
        .iter()
        .enumerate()
        .find_map(|(r, row)| {
            row.iter()
                .enumerate()
                .find(|(_, c)| **c == 'S')
                .map(|(col, _)| Coord(r, col))
        })
        .unwrap();

    let mut to_search: HashSet<Coord> = HashSet::new();

    to_search.insert(start);

    // while let Some(c) = to_search {
    //     // let nbrs = [
    //     // ].filter(
    //     break;
    // }
    todo!()
}

fn run_2(_input: &str) -> anyhow::Result<usize> {
    todo!()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coord(usize, usize);

impl Coord {
    fn col(&self) -> usize {
        self.1
    }
    fn icol(&self) -> isize {
        self.1 as _
    }

    fn row(&self) -> usize {
        self.0
    }
    fn irow(&self) -> isize {
        self.0 as _
    }
}
type Map = Vec<Vec<char>>;

fn can_go(from: &Coord, to: Coord, map: &Map) -> bool {
    if !map
        .get(to.row())
        .map(|row| row.get(to.col()))
        .flatten()
        .is_some()
    {
        return false;
    }
    match (to.irow() - from.irow(), to.icol() - from.icol()) {
        // Coming from north
        (1, 0) => match map[to.row()][to.col()] {
            '|' | 'L' | 'J' => true,
            _ => false,
        },
        // Coming from south
        (-1, 0) => match map[to.row()][to.col()] {
            '|' | '7' | 'F' => true,
            _ => false,
        },
        // coming from west
        (0, 1) => match map[to.row()][to.col()] {
            '-' | 'J' | '7' => true,
            _ => false,
        },
        // coming from east
        (0, -1) => match map[to.row()][to.col()] {
            '-' | 'L' | 'F' => true,
            _ => false,
        },
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    const INPUT_1: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

    #[test]
    fn day10_run_1() {
        assert_eq!(super::run_1(INPUT_1).unwrap(), 8);
    }

    #[test]
    fn day10_run_2() {}
}
