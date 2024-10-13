use std::collections::HashSet;

use crate::common::Coord;

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day21.txt")?;

    let a:isize = -1;
    println!("a {}", a % 10);

    println!("21:1 - {}", run_1(&input)?);
    println!("21:2 - {}", run_2(&input)?);

    Ok(())
}

fn parse(i: &str) -> (Coord, HashSet<Coord>) {
    let mut start = None;
    let mut garden_plots = HashSet::new();
    i.lines().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, c)| match c {
            '.' => {
                garden_plots.insert((row, col).into());
            }
            'S' => {
                garden_plots.insert((row, col).into());
                start = Some((row, col).into())
            }
            _ => (),
        })
    });
    (start.unwrap(), garden_plots)
}

fn print(map: &HashSet<Coord>, overlay: &HashSet<Coord>) {
    let (max_row, max_col) = map.iter().fold((0, 0), |(max_row, max_col), c| {
        (max_row.max(c.row()), max_col.max(c.col()))
    });

    for r in 0..=max_row {
        for c in 0..=max_col {
            let c: Coord = (r, c).into();
            if overlay.contains(&c) {
                print!("O");
            } else if map.contains(&c) {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!()
    }
}

fn simulate(i: &str, steps: usize) -> anyhow::Result<usize> {
    let (start, garden_plots) = parse(i);
    let mut visited = HashSet::new();
    visited.insert(start);

    let mut this_round = HashSet::new();
    this_round.insert(start);

    for _ in 0..steps {
        let mut next_round = HashSet::new();
        for c in this_round {
            let nbrs: Vec<_> = c
                .neighbors()
                .filter(
                    |c| garden_plots.contains(c), //&& !visited.contains(c)
                )
                .collect();

            nbrs.iter().for_each(|c| {
                visited.insert(*c);
                next_round.insert(*c);
            });
        }

        this_round = next_round;
    }

    Ok(this_round.len())
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    simulate(input, 64)
}

fn simulate_2(i: &str, steps: usize) -> anyhow::Result<usize> {
    let (start, garden_plots) = parse(i);
    let mut visited = HashSet::new();
    visited.insert(start);

    let mut this_round = HashSet::new();
    this_round.insert(start);

    for _ in 0..steps {
        let mut next_round = HashSet::new();
        for c in this_round {
            let nbrs: Vec<_> = c
                .neighbors()
                .filter(
                    |c| garden_plots.contains(c), //&& !visited.contains(c)
                )
                .collect();

            nbrs.iter().for_each(|c| {
                visited.insert(*c);
                next_round.insert(*c);
            });
        }

        this_round = next_round;
    }

    dbg! {&this_round};
    Ok(this_round.len())
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    simulate_2(input, 26_501_365)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn day21_run_1() {
        assert_eq!(super::simulate(INPUT, 1).unwrap(), 2);
        assert_eq!(super::simulate(INPUT, 2).unwrap(), 4);
        assert_eq!(super::simulate(INPUT, 3).unwrap(), 6);
        assert_eq!(super::simulate(INPUT, 6).unwrap(), 16);
    }

    #[test]
    fn day21_run_2() {
        assert_eq!(super::simulate_2(INPUT, 6).unwrap(), 16);
        //assert_eq!(super::simulate_2(INPUT, 10).unwrap(), 50);
        //assert_eq!(super::simulate_2(INPUT, 50).unwrap(), 1594);
        //assert_eq!(super::simulate_2(INPUT, 100).unwrap(), 6536);
        //assert_eq!(super::simulate_2(INPUT, 500).unwrap(), 167004);
        //assert_eq!(super::simulate_2(INPUT, 1000).unwrap(), 668697);
        //assert_eq!(super::simulate_2(INPUT, 5000).unwrap(), 16733044);
    }
}
