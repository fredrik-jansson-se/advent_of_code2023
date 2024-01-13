use num::Integer;

use crate::common::Coord;
use std::collections::HashSet;

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
                .map(|(col, _)| (r, col).into())
        })
        .unwrap();

    let mut to_search: Vec<Coord> = Vec::new();
    let mut visited: HashSet<Coord> = HashSet::new();

    to_search.push(start);

    while let Some(cur) = to_search.pop() {
        if visited.contains(&cur) {
            continue;
        }
        visited.insert(cur);

        let nbrs = cur
            .neighbors()
            .filter(|coord| !visited.contains(coord) && can_go(&cur, coord, &map));

        to_search.extend(nbrs);
    }

    Ok(visited.len() / 2)
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let map: Map = input.lines().map(|row| row.chars().collect()).collect();

    let start: Coord = map
        .iter()
        .enumerate()
        .find_map(|(r, row)| {
            row.iter()
                .enumerate()
                .find(|(_, c)| **c == 'S')
                .map(|(col, _)| (r, col).into())
        })
        .unwrap();

    let mut to_search: Vec<Coord> = Vec::new();
    let mut visited: HashSet<Coord> = HashSet::new();

    to_search.push(start);

    while let Some(cur) = to_search.pop() {
        if visited.contains(&cur) {
            continue;
        }
        visited.insert(cur);

        let nbrs = cur
            .neighbors()
            .filter(|coord| !visited.contains(coord) && can_go(&cur, coord, &map));

        to_search.extend(nbrs);
    }

    for (r, row) in map.iter().enumerate() {
        for (col, c) in row.iter().enumerate() {
            let coord: Coord = (r, col).into();
            if visited.contains(&coord) {
                print!("{c}");
            } else {
                print!(".");
            }
        }
        println!();
    }

    // let mut inside_cnt = 0;
    let mut per_row_count = HashSet::new();
    for (r, row) in map.iter().enumerate() {
        let mut hit_cnt = 0;
        // let last_col = row.len() -  row.iter().rev().position(|c| *c != '.').unwrap_or(row.len());
        let last_col = row.len();
        println!("r: {r} - {last_col}");
        for c in 0..last_col {
            let map_item = map[r][c];
            let cur: Coord = (r, c).into();

            // println!("{cur} -> {hit_cnt}");
            match map_item {
                '|' | 'F' | 'S' | 'L' | 'J' | '7' => {
                    hit_cnt += 1;
                }
                '.' if hit_cnt.is_odd() => {
                    println!("Adding {cur}");
                    per_row_count.insert(cur);
                }
                _ => (),
            }
            // println!("{cur} <- {hit_cnt}");
        }
        println!("r: {r} hit_cnt: {hit_cnt}");
    }

    // let mut per_col_count = HashSet::new();
    // for c in 0..map[0].len() {
    //     let mut hit_cnt = 0;
    //     for (r, row) in map.iter().enumerate() {
    //         let map_item = row[c];
    //         let cur: Coord = (r, c).into();

    //         // println!("{cur} -> {hit_cnt}");
    //         match map_item {
    //             '-' | 'F' | 'S' | 'L' | '7' | 'J' => {
    //                 hit_cnt += 1;
    //             }
    //             '.' if hit_cnt.is_odd() => {
    //                 println!("Adding {cur}");
    //                 per_col_count.insert(cur);
    //             }
    //             _ => (),
    //         }
    //         // println!("{cur} <- {hit_cnt}");
    //     }
    // }
    println!("per row: {}", per_row_count.len());
    // println!("per col: {}", per_col_count.len());

    // Ok(per_row_count.intersection(&per_col_count).count())
    Ok(per_row_count.len())
}

type Map = Vec<Vec<char>>;

fn can_go(from: &Coord, to: &Coord, map: &Map) -> bool {
    if map
        .get(to.row())
        .and_then(|row| row.get(to.col()))
        .is_none()
    {
        return false;
    }
    let src = map[from.row()][from.col()];
    match (to.irow() - from.irow(), to.icol() - from.icol()) {
        // Coming from north
        (1, 0) => {
            (src == '|' || src == '7' || src == 'F' || src == 'S')
                && matches!(map[to.row()][to.col()], '|' | 'L' | 'J')
        }
        // Coming from south
        (-1, 0) => {
            (src == '|' || src == 'L' || src == 'J' || src == 'S')
                && matches!(map[to.row()][to.col()], '|' | '7' | 'F')
        }
        // coming from west
        (0, 1) => {
            (src == '-' || src == 'L' || src == 'F' || src == 'S')
                && matches!(map[to.row()][to.col()], '-' | 'J' | '7')
        }
        // coming from east
        (0, -1) => {
            (src == '-' || src == 'J' || src == '7' || src == 'S')
                && matches!(map[to.row()][to.col()], '-' | 'L' | 'F')
        }
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
    const INPUT_2: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    #[test]
    fn day10_run_1() {
        assert_eq!(super::run_1(INPUT_1).unwrap(), 4);
        assert_eq!(super::run_1(INPUT_2).unwrap(), 8);
    }

    #[test]
    #[ignore]
    fn day10_run_2() {
        // assert_eq!(super::run_2(".S.").unwrap(), 1);
        // assert_eq!(super::run_2(".S-.").unwrap(), 1);
        // assert_eq!(
        //     super::run_2(
        //         ".F7.
        // .S|."
        //     )
        //     .unwrap(),
        //     0
        // );
        assert_eq!(
            super::run_2(
                "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."
            )
            .unwrap(),
            4
        );
        assert_eq!(
            super::run_2(
                ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."
            )
            .unwrap(),
            8
        );

        assert_eq!(
            super::run_2(
                "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"
            )
            .unwrap(),
            10
        );
    }
}
