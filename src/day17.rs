// use crate::common::Dir;

use crate::common::{Coord, Dir, Pos};

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day17.txt")?;

    println!("17:1 - {}", run_1(&input)?);
    println!("17:2 - {}", run_2(&input)?);

    Ok(())
}

#[derive(Clone, Debug)]
struct Crucible {
    pos: Pos,
    num_steps_forward: usize,
}

impl Crucible {
    fn move_forward(&self) -> Self {
        let pos = self.pos.move_forward();
        Self {
            pos,
            num_steps_forward: self.num_steps_forward + 1,
        }
    }

    fn turn_right(&self) -> Self {
        let pos = self.pos.turn_right().move_forward();
        Self {
            pos,
            num_steps_forward: 1,
        }
    }
    fn turn_left(&self) -> Self {
        let pos = self.pos.turn_left().move_forward();
        Self {
            pos,
            num_steps_forward: 1,
        }
    }
}

impl PartialEq for Crucible {
    fn eq(&self, other: &Self) -> bool {
        self.pos.c == other.pos.c
    }
}

impl std::hash::Hash for Crucible {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.c.hash(state);
    }
}

impl Eq for Crucible {}

// Wrong: 825
// Too high: 822
// Too low: 778
// Too high: 805
fn run_1(input: &str) -> anyhow::Result<usize> {
    let map: Vec<Vec<isize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as isize)
                .collect()
        })
        .collect();

    fn possible_moves(c: &Crucible, map: &[Vec<isize>]) -> Vec<(Crucible, isize)> {
        let in_map = |c: &Crucible| map.get(c.pos.row()).and_then(|row| row.get(c.pos.col()));
        let mut res = Vec::with_capacity(3);

        // Can only move forward three times in a row
        if c.num_steps_forward < 3 {
            let n = c.move_forward();
            if let Some(cost) = in_map(&n) {
                res.push((n, *cost));
            }
        }

        let n = c.turn_right();
        if let Some(cost) = in_map(&n) {
            res.push((n, *cost));
        }

        let n = c.turn_left();
        if let Some(cost) = in_map(&n) {
            res.push((n, *cost));
        }

        if (c.pos.row() < 1 && c.pos.col() < 3) || (c.pos.c == (1, 2).into()) {
            dbg! {&c};
            dbg! {&res};
        }

        res
    }

    let start = Crucible {
        pos: Pos {
            c: (0, 0).into(),
            dir: Dir::E,
        },
        num_steps_forward: 0,
    };

    let finish: Coord = (
        (map.len() - 1) as isize,
        (map[map.len() - 1].len() - 1) as isize,
    )
        .into();

    let (path, cost) = pathfinding::directed::astar::astar(
        &start,
        |crucible| possible_moves(crucible, &map),
        |crucible| crucible.pos.c.manhattan(&finish) as isize,
        |crucible| crucible.pos.c == finish,
    )
    .unwrap();

    for (ri, row) in map.iter().enumerate() {
        for (ci, c) in row.iter().enumerate() {
            if let Some(crucible) = path.iter().find(|c| c.pos.c == (ri, ci).into()) {
                match crucible.pos.dir {
                    Dir::N => print!("^"),
                    Dir::S => print!("v"),
                    Dir::E => print!(">"),
                    Dir::W => print!("<"),
                }
            } else {
                print!("{c}");
            }
        }
        println!();
    }
    // dbg!{&path};

    // let cost = cost - map[finish.row()][finish.col()];
    Ok(cost as usize)
}

fn run_2(_input: &str) -> anyhow::Result<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn day17_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 102);
    }

    #[test]
    fn day17_run_2() {}
}
