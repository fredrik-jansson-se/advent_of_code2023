use std::{collections::HashMap, usize};
use rayon::prelude::*;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::separated_list1,
};

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day19.txt")?;

    println!("19:1 - {}", run_1(&input)?);
    println!("19:2 - {}", run_2(&input)?);

    Ok(())
}

#[derive(Debug)]
struct MachinePart {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl MachinePart {
    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, PartialEq, Eq)]
enum RuleRes {
    Accepted,
    Rejected,
    Next(String),
    NoMatch,
}

// #[derive(Debug)]
struct System {
    workflows: HashMap<String, Vec<Rule>>,
    parts: Vec<MachinePart>,
}

type Rule = Box<dyn Fn(&MachinePart) -> RuleRes + Sync>;

fn parse_complex_rule(i: crate::Input) -> crate::PResult<Rule> {
    let lt = nom::combinator::map(tag("<"), |_| true);
    let gt = nom::combinator::map(tag(">"), |_| false);

    let (i, var) = nom::combinator::map(alpha1, |s: &str| s.to_string())(i)?;
    let (i, lt) = nom::branch::alt((lt, gt))(i)?;
    let (i, num) = nom::combinator::map(nom::character::complete::u64, |v| v as usize)(i)?;

    let (i, _) = tag(":")(i)?;
    let (i, next) = nom::combinator::map(alpha1, |s: &str| s.to_string())(i)?;

    let f = Box::new(move |m: &MachinePart| match (var.as_str(), lt) {
        ("x", true) if m.x < num => RuleRes::Next(next.clone()),
        ("x", false) if m.x > num => RuleRes::Next(next.clone()),
        ("m", true) if m.m < num => RuleRes::Next(next.clone()),
        ("m", false) if m.m > num => RuleRes::Next(next.clone()),
        ("a", true) if m.a < num => RuleRes::Next(next.clone()),
        ("a", false) if m.a > num => RuleRes::Next(next.clone()),
        ("s", true) if m.s < num => RuleRes::Next(next.clone()),
        ("s", false) if m.s > num => RuleRes::Next(next.clone()),
        _ => RuleRes::NoMatch,
    });

    Ok((i, f))
}
fn parse_simple_rule(i: crate::Input) -> crate::PResult<Rule> {
    let (i, v) = alpha1(i)?;
    match v {
        "A" => Ok((i, Box::new(|_: &MachinePart| RuleRes::Accepted))),
        "R" => Ok((i, Box::new(|_: &MachinePart| RuleRes::Rejected))),
        v => {
            let v = v.to_string();
            Ok((i, Box::new(move |_: &MachinePart| RuleRes::Next(v.clone()))))
        }
    }
}

fn parse_workflow(i: crate::Input) -> crate::PResult<(String, Vec<Rule>)> {
    let (i, name) = alpha1(i)?;
    let (i, _) = tag("{")(i)?;
    let (i, rules) = separated_list1(
        tag(","),
        nom::branch::alt((parse_complex_rule, parse_simple_rule)),
    )(i)?;
    let (i, _) = tag("}")(i)?;
    Ok((i, (name.to_owned(), rules)))
}

fn parse_machine_part(i: crate::Input) -> crate::PResult<MachinePart> {
    let (i, _) = tag("{")(i)?;
    let (i, _) = tag("x=")(i)?;
    let (i, x) = nom::combinator::map(nom::character::complete::u64, |v| v as usize)(i)?;
    let (i, _) = tag(",m=")(i)?;
    let (i, m) = nom::combinator::map(nom::character::complete::u64, |v| v as usize)(i)?;
    let (i, _) = tag(",a=")(i)?;
    let (i, a) = nom::combinator::map(nom::character::complete::u64, |v| v as usize)(i)?;
    let (i, _) = tag(",s=")(i)?;
    let (i, s) = nom::combinator::map(nom::character::complete::u64, |v| v as usize)(i)?;
    let (i, _) = tag("}")(i)?;
    Ok((i, MachinePart { x, m, a, s }))
}

fn parse(i: crate::Input) -> crate::PResult<System> {
    let (i, wfs) = separated_list1(newline, parse_workflow)(i)?;

    let (i, _) = newline(i)?;
    let (i, _) = newline(i)?;

    let (i, parts) = separated_list1(newline, parse_machine_part)(i)?;

    let workflows = wfs.into_iter().collect();

    Ok((i, System { workflows, parts }))
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let (_, system) = parse(input).map_err(|e| anyhow::anyhow!("{e}"))?;

    let mut accepted = Vec::new();

    for part in system.parts {
        let mut cur = "in".to_string();
        'part_loop: loop {
            let rules = system.workflows.get(&cur).unwrap();
            for rule in rules.iter() {
                match rule(&part) {
                    RuleRes::Accepted => {
                        accepted.push(part);
                        break 'part_loop;
                    }
                    RuleRes::Rejected => {
                        break 'part_loop;
                    }
                    RuleRes::Next(n) if n == "A" => {
                        accepted.push(part);
                        break 'part_loop;
                    }
                    RuleRes::Next(n) if n == "R" => {
                        break 'part_loop;
                    }
                    RuleRes::Next(n) => {
                        cur = n.clone();
                        break;
                    }
                    RuleRes::NoMatch => {
                        continue;
                    }
                }
            }
        }
    }

    Ok(accepted.iter().map(|m| m.sum()).sum())
}

fn is_accepted(part: MachinePart, wfs: &HashMap<String, Vec<Rule>>) -> bool {
    let mut cur = "in".to_string();
    loop {
        let rules = wfs.get(&cur).unwrap();
        for rule in rules.iter() {
            match rule(&part) {
                RuleRes::Accepted => {
                    return true;
                }
                RuleRes::Rejected => {
                    return false;
                }
                RuleRes::Next(n) if n == "A" => {
                    return true;
                }
                RuleRes::Next(n) if n == "R" => {
                    return false;
                }
                RuleRes::Next(n) => {
                    cur = n.clone();
                    break;
                }
                RuleRes::NoMatch => {
                    continue;
                }
            }
        }
    }
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let (_, system) = parse(input).map_err(|e| anyhow::anyhow!("{e}"))?;

    let (xmin, xmax) = (1..4001).fold((usize::MAX, usize::MIN), |(min, max), x| {
        if is_accepted(
            MachinePart {
                x,
                m: 0,
                a: 0,
                s: 0,
            },
            &system.workflows,
        ) {
            (min.min(x), max.max(x))
        } else {
            (min, max)
        }
    });
    println!("x: {xmin} {xmax}");

    let (mmin, mmax) = (1..4001).fold((usize::MAX, usize::MIN), |(min, max), m| {
        if is_accepted(
            MachinePart {
                x: 0,
                m,
                a: 0,
                s: 0,
            },
            &system.workflows,
        ) {
            (min.min(m), max.max(m))
        } else {
            (min, max)
        }
    });
    println!("m: {mmin} {mmax}");

    let (amin, amax) = (1..4001).fold((usize::MAX, usize::MIN), |(min, max), a| {
        if is_accepted(
            MachinePart {
                x: 0,
                m: 0,
                a,
                s: 0,
            },
            &system.workflows,
        ) {
            (min.min(a), max.max(a))
        } else {
            (min, max)
        }
    });
    println!("a: {amin} {amax}");

    let (smin, smax) = (1..4001).fold((usize::MAX, usize::MIN), |(min, max), s| {
        if is_accepted(
            MachinePart {
                x: 0,
                m: 0,
                a: 0,
                s,
            },
            &system.workflows,
        ) {
            (min.min(s), max.max(s))
        } else {
            (min, max)
        }
    });
    println!("s: {smin} {smax}");

   
    // let tot = (xmin..=xmax).into_par_iter().map(|x| {
    //     (mmin..=mmax).map(|m| {
    //         (amin..=amax).map(|a| {
    //             (smin..=smax).map(|s| {
    //                 if is_accepted(MachinePart{x,m,a,s}, &system.workflows) {
    //                     1
    //                 }
    //                 else {
    //                     0
    //                 }
    //             }).sum::<usize>()
    //         }).sum::<usize>()
    //     }).sum::<usize>()
    // }).sum();

    Ok((xmax - xmin + 1) * (mmax - mmin + 1) * (amax-amin+1) * (smax-smin+1))
    // Ok(tot)
}

#[cfg(test)]
mod tests {
    use crate::day19::RuleRes;

    const INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn day19_parse() {
        let (i, f) = super::parse_complex_rule("a<2006:qkq").unwrap();
        assert_eq!(i, "");
        let v = super::MachinePart {
            x: 10,
            m: 20,
            a: 30,
            s: 40,
        };
        assert_eq!(f(&v), RuleRes::Next("qkq".to_string()));
        let v = super::MachinePart {
            x: 10,
            m: 20,
            a: 3000,
            s: 40,
        };
        assert_eq!(f(&v), RuleRes::NoMatch);
    }

    #[test]
    fn day19_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 19114);
    }

    #[test]
    fn day19_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 167409079868000);
    }
}
