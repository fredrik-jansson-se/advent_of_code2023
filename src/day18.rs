use std::collections::HashSet;

use crate::{
    common::{Coord, Dir},
    Input, PResult,
};

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day18.txt")?;

    println!("18:1 - {}", run_1(&input)?);
    println!("18:2 - {}", run_2(&input)?);

    Ok(())
}

#[derive(Debug)]
struct Dig {
    dir: Dir,
    steps: isize,
    color: usize,
}

// 42921 -- too high
fn run_1(input: &str) -> anyhow::Result<usize> {
    let (_, digs) = parse(input).map_err(|e| anyhow::anyhow!("{e}"))?;

    let mut sides: HashSet<crate::common::Coord> = Default::default();
    let mut last = (0, 0).into();
    for dig in digs {
        let mut cur = last;
        sides.insert(cur);
        for _ in 0..dig.steps {
            cur = cur + dig.dir.movement();
            sides.insert(cur);
        }
        last = cur;
    }

    let seed = (1, 1).into();
    assert!(!sides.contains(&seed));
    let interiors = find_interiors(seed, &sides);
    sides.extend(interiors);

    Ok(sides.len())
}

fn find_interiors(seed: Coord, sides: &HashSet<Coord>) -> std::collections::HashSet<Coord> {
    let mut to_search = HashSet::new();
    to_search.insert(seed);
    let mut res: std::collections::HashSet<Coord> = Default::default();
    while let Some(c) = to_search.iter().cloned().next() {
        to_search.remove(&c);

        if sides.contains(&c) {
            continue;
        }
        res.insert(c);
        let nbrs: Vec<_> = [Dir::N, Dir::E, Dir::S, Dir::W]
            .into_iter()
            .map(|d| c + d.movement())
            .filter(|n| !res.contains(n))
            .collect();

        to_search.extend(nbrs);
    }

    res
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let (_, digs) = parse(input).map_err(|e| anyhow::anyhow!("{e}"))?;

    let mut sides: HashSet<crate::common::Coord> = Default::default();
    let mut last = (0, 0).into();
    for dig in digs {
        let cur = last;
        sides.insert(cur);

        let dir = match dig.color & 0xf {
            0 => Dir::E,
            1 => Dir::S,
            2 => Dir::W,
            3 => Dir::N,
            _ => unreachable!(),
        };
        let steps = dig.color >> 4;
        println!("steps: {}, {:?}", steps, dir);

        //panic!();
        //for _ in 0..steps {
        //    cur = cur + dir.movement();
        //    sides.insert(cur);
        //}
        last = cur;
    }

    let seed = (1, 1).into();
    assert!(!sides.contains(&seed));
    //let interiors = find_interiors(seed, &sides);
    //sides.extend(interiors);

    Ok(sides.len())
}

fn parse_dir(i: Input) -> PResult<Dir> {
    let down = nom::combinator::map(nom::bytes::complete::tag("D"), |_| Dir::S);
    let up = nom::combinator::map(nom::bytes::complete::tag("U"), |_| Dir::N);
    let left = nom::combinator::map(nom::bytes::complete::tag("L"), |_| Dir::W);
    let right = nom::combinator::map(nom::bytes::complete::tag("R"), |_| Dir::E);
    let res = nom::branch::alt((down, up, left, right))(i)?;
    Ok(res)
}

fn parse_color(i: Input) -> PResult<usize> {
    let (i, _) = nom::bytes::complete::tag("(#")(i)?;
    let (i, color) =
        nom::combinator::map_res(nom::character::complete::alphanumeric1, |c: &str| {
            usize::from_str_radix(c, 16)
        })(i)?;
    let (i, _) = nom::bytes::complete::tag(")")(i)?;
    Ok((i, color))
}

fn parse_dig(i: Input) -> PResult<Dig> {
    let (i, dir) = parse_dir(i)?;
    let (i, _) = nom::character::complete::space1(i)?;
    let (i, steps) = nom::combinator::map(nom::character::complete::u64, |v| v as isize)(i)?;
    let (i, _) = nom::character::complete::space1(i)?;
    let (i, color) = parse_color(i)?;
    Ok((i, Dig { dir, steps, color }))
}

fn parse(i: Input) -> PResult<Vec<Dig>> {
    nom::multi::separated_list1(nom::character::complete::newline, parse_dig)(i)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn day18_dir() {
        assert_eq!(super::parse_dir("D").unwrap().1, crate::common::Dir::S);
        assert_eq!(super::parse_dir("U").unwrap().1, crate::common::Dir::N);
        assert_eq!(super::parse_dir("L").unwrap().1, crate::common::Dir::W);
        assert_eq!(super::parse_dir("R").unwrap().1, crate::common::Dir::E);
    }

    #[test]
    fn day18_color() {
        assert_eq!(super::parse_color("(#0dc571)").unwrap().1, 0xdc571);
        assert_eq!(super::parse_color("(#7a21e3)").unwrap().1, 0x7a21e3);
    }
    #[test]
    fn day18_parse() {
        assert_eq!(super::parse(INPUT).unwrap().1.len(), 14);
    }

    #[test]
    fn day18_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 62);
    }

    #[test]
    fn day18_interiors() {
        let mut sides: std::collections::HashSet<crate::common::Coord> = Default::default();
        sides.insert((0, 0).into());
        sides.insert((0, 1).into());
        sides.insert((0, 2).into());
        sides.insert((0, 3).into());
        sides.insert((1, 0).into());
        sides.insert((1, 3).into());
        sides.insert((2, 0).into());
        sides.insert((2, 3).into());
        sides.insert((3, 0).into());
        sides.insert((3, 1).into());
        sides.insert((3, 2).into());
        sides.insert((3, 3).into());
        let ints = super::find_interiors((1, 1).into(), &sides);
        assert_eq!(ints.len(), 4);
    }

    #[test]
    #[ignore]
    fn day18_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 952408144115);
    }
}
