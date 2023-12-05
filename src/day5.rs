use std::{collections::HashMap, usize};
use rayon::prelude::*;

use nom::{
    bytes::complete::tag,
    character::complete::{newline, space1},
    multi::separated_list1,
};

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day5.txt")?;

    println!("5:1 - {}", run_1(&input)?);
    println!("5:2 - {}", run_2(&input)?);

    Ok(())
}

#[derive(Debug)]
struct Garden {
    seeds: Vec<usize>,
    maps: Vec<(String, String, Vec<Map>)>,
}

fn next_dest(maps: &[Map], src_value: usize) -> usize {
    maps.iter()
        .find_map(|m| m.to_dest(src_value))
        .unwrap_or(src_value)
}

impl Garden {
}

#[derive(Debug)]
struct Map {
    dest_range: std::ops::Range<usize>,
    src_range: std::ops::Range<usize>,
}

impl Map {
    fn new(dest: usize, src: usize, len: usize) -> Self {
        Self {
            dest_range: dest..(dest + len),
            src_range: src..(src + len),
        }
    }
    fn to_dest(&self, source: usize) -> Option<usize> {
        if self.src_range.contains(&source) {
            Some(self.dest_range.start + source - self.src_range.start)
        } else {
            None
        }
    }
}

fn parse_map(i: crate::Input) -> crate::PResult<Map> {
    let (i, map) = nom::multi::many_m_n(
        3,
        3,
        nom::sequence::terminated(
            nom::character::complete::u32,
            nom::character::complete::space0,
        ),
    )(i)?;
    Ok((i, Map::new(map[0] as _, map[1] as _, map[2] as _)))
}

fn parse_maps(i: crate::Input) -> crate::PResult<(String, String, Vec<Map>)> {
    let (i, (from, to)) = nom::sequence::separated_pair(
        nom::character::complete::alpha1,
        tag("-to-"),
        nom::character::complete::alpha1,
    )(i)?;
    let (i, _) = tag(" map:")(i)?;
    let (i, _) = newline(i)?;
    let (i, maps) = separated_list1(newline, parse_map)(i)?;
    let (i, _) = nom::combinator::opt(newline)(i)?;
    Ok((i, (from.to_string(), to.to_string(), maps)))
}

fn parse(i: crate::Input) -> crate::PResult<Garden> {
    // Parse seeds
    let (i, _) = tag("seeds: ")(i)?;
    let (i, seeds) = separated_list1(
        space1,
        nom::combinator::map(nom::character::complete::u32, |v| v as usize),
    )(i)?;
    // Eat two newlines
    let (i, _) = newline(i)?;
    let (i, _) = newline(i)?;

    let (i, maps) = separated_list1(newline, parse_maps)(i)?;

    Ok((i, Garden { seeds, maps }))
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let (_, garden) = parse(input).map_err(|e| anyhow::anyhow!(e.to_string()))?;
    let mut results = HashMap::new();
    let end = "location";
    let mut cur = "seed";

    results.insert(cur, garden.seeds.clone());

    loop {
        let (_, next, maps) = garden.maps.iter().find(|(s, _, _)| s == cur).unwrap();

        let res = results[cur]
            .iter()
            .map(|c| next_dest(maps, *c))
            .collect();

        results.insert(next, res);

        if next == end {
            break;
        }
        results.remove(cur);
        cur = next;
    }

    Ok(*results[end].iter().min().unwrap())
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let (_, garden) = parse(input).map_err(|e| anyhow::anyhow!(e.to_string()))?;
    let mut results = HashMap::new();
    let end = "location";
    let mut cur = "seed";

    let seeds:&mut Vec<usize> = results.entry(cur).or_default();

    for s in garden.seeds.chunks(2) {
        let seed_start = s[0];
        let seed_len = s[1];
        for i in 0..seed_len {
            seeds.push(seed_start+i);
        }
    }

    loop {
        let (_, next, maps) = garden.maps.iter().find(|(s, _, _)| s == cur).unwrap();

        let res = results[cur]
            .par_iter()
            .map(|c| next_dest(maps, *c))
            .collect();

        results.insert(next, res);

        if next == end {
            break;
        }
        results.remove(cur);
        cur = next;
    }

    Ok(*results[end].iter().min().unwrap())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn day5_map() {
        let map = super::Map::new(20, 10, 5);
        assert_eq!(map.to_dest(10), Some(20));
        assert_eq!(map.to_dest(14), Some(24));
        assert_eq!(map.to_dest(15), None);
        assert_eq!(map.to_dest(9), None);
    }

    #[test]
    fn day5_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 35);
    }

    #[test]
    fn day5_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 46);
    }
}
