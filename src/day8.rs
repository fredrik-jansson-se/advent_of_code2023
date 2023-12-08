use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::newline,
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair, terminated},
};

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day8.txt")?;

    println!("8:1 - {}", run_1(&input)?);
    println!("8:2 - {}", run_2(&input)?);

    Ok(())
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let (_i, document) = parse(input).map_err(|e| anyhow::anyhow!(e.to_string()))?;

    let mut cur = "AAA";
    let mut iterations = 0;
    for i in document.instructions.iter().cycle() {
        if cur == "ZZZ" {
            break;
        }
        iterations += 1;
        let possibles = document.network.get(cur).unwrap();
        cur = match i {
            Instruction::Left => &possibles.0,
            Instruction::Right => &possibles.1,
        }
    }
    Ok(iterations)
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let (_i, document) = parse(input).map_err(|e| anyhow::anyhow!(e.to_string()))?;

    let mut cur: Vec<String> = document
        .network
        .keys()
        .filter_map(|n| {
            if n.ends_with('A') {
                Some(n.to_string())
            } else {
                None
            }
        })
        .collect();

    let mut res = Vec::with_capacity(cur.len());
    for c in cur.iter_mut() {
        let mut iterations = 0;
        for i in document.instructions.iter().cycle() {
            if c.ends_with('Z') {
                break;
            }
            iterations += 1;
            let possibles = document.network.get(c).unwrap();
            match i {
                Instruction::Left => *c = possibles.0.clone(),
                Instruction::Right => *c = possibles.1.clone(),
            };
        }
        res.push(iterations);
    }

    Ok(lcm(&res))
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug)]
struct Document {
    instructions: Vec<Instruction>,
    network: HashMap<String, (String, String)>,
}

fn parse_network(i: crate::Input) -> crate::PResult<(String, (String, String))> {
    let parse_str = |i| {
        nom::combinator::map(nom::character::complete::alphanumeric1, |c: &str| {
            c.to_string()
        })(i)
    };

    let res = separated_pair(
        parse_str,
        tag(" = "),
        separated_pair(
            preceded(tag("("), parse_str),
            tag(", "),
            terminated(parse_str, tag(")")),
        ),
    )(i)?;

    Ok(res)
}

fn parse(i: crate::Input) -> crate::PResult<Document> {
    let left = nom::combinator::map(tag("L"), |_| Instruction::Left);
    let right = nom::combinator::map(tag("R"), |_| Instruction::Right);
    let (i, instructions) = many1(nom::branch::alt((left, right)))(i)?;

    let (i, _) = newline(i)?;
    let (i, _) = newline(i)?;

    let (i, network) = nom::combinator::map(separated_list1(newline, parse_network), |v| {
        v.iter()
            .map(|(a, (b, c))| (a.to_string(), (b.to_string(), c.to_string())))
            .collect::<HashMap<_, _>>()
    })(i)?;

    Ok((
        i,
        Document {
            instructions,
            network,
        },
    ))
}

#[cfg(test)]
mod tests {
    const INPUT_1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const INPUT_2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn day8_run_1() {
        assert_eq!(super::run_1(INPUT_1).unwrap(), 2);
        assert_eq!(super::run_1(INPUT_2).unwrap(), 6);
    }

    const INPUT_3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn day8_run_2() {
        assert_eq!(super::run_2(INPUT_3).unwrap(), 6);
    }
}
