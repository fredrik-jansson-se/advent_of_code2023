use rayon::prelude::*;
use std::collections::HashSet;

use crate::common::{Dir, Pos};

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day16.txt")?;

    println!("16:1 - {}", run_1(&input)?);
    println!("16:2 - {}", run_2(&input)?);

    Ok(())
}

fn calc_energized(map: &[Vec<char>], ray: Pos) -> usize {
    let mut rays = vec![ray];
    let mut visited = HashSet::new();
    let mut new_rays = Vec::new();
    let mut added = true;
    while added {
        added = false;
        for ray in rays.iter_mut() {
            if visited.contains(ray) {
                continue;
            }
            if ray.row() < map.len() && ray.col() < map[ray.row()].len() {
                visited.insert(*ray);
                added = true;

                match map[ray.row()][ray.col()] {
                    '/' if (ray.dir == Dir::E || ray.dir == Dir::W) => {
                        ray.dir = ray.dir.turn_left();
                    }
                    '/' if (ray.dir == Dir::S || ray.dir == Dir::N) => {
                        ray.dir = ray.dir.turn_right();
                    }
                    '\\' if (ray.dir == Dir::E || ray.dir == Dir::W) => {
                        ray.dir = ray.dir.turn_right();
                    }
                    '\\' if (ray.dir == Dir::S || ray.dir == Dir::N) => {
                        ray.dir = ray.dir.turn_left();
                    }
                    '|' if (ray.dir == Dir::E || ray.dir == Dir::W) => {
                        let mut new = *ray;
                        ray.dir = ray.dir.turn_left();
                        new.dir = new.dir.turn_right();
                        new_rays.push(new);
                    }
                    '|' => (),
                    '-' if (ray.dir == Dir::N || ray.dir == Dir::S) => {
                        let mut new = *ray;
                        ray.dir = ray.dir.turn_left();
                        new.dir = new.dir.turn_right();
                        new_rays.push(new);
                    }
                    '-' => (),
                    '.' => (),
                    _ => {
                        unreachable!();
                    }
                }

                let next = ray.move_forward();

                *ray = next;
            }
        }
        rays.append(&mut new_rays);
    }

    let visited: HashSet<_> = visited.into_iter().map(|p| p.c).collect();
    visited.len()
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    Ok(calc_energized(
        &map,
        Pos {
            dir: Dir::E,
            c: (0, 0).into(),
        },
    ))
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = map.len();
    let cols = map[0].len();

    let mut starting_points = Vec::new();
    for row in 0..rows {
        starting_points.push(Pos {
            dir: Dir::E,
            c: (row, 0).into(),
        });
        starting_points.push(Pos {
            dir: Dir::W,
            c: (row, cols - 1).into(),
        });
    }
    for col in 0..cols {
        starting_points.push(Pos {
            dir: Dir::S,
            c: (0, col).into(),
        });
        starting_points.push(Pos {
            dir: Dir::N,
            c: (rows - 1, col).into(),
        });
    }
    Ok(starting_points
        .par_iter()
        .map(|ray| calc_energized(&map, *ray))
        .max()
        .unwrap())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn day16_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 46);
    }

    #[test]
    fn day16_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 51);
    }
}
