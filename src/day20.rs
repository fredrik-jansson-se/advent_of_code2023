use std::collections::HashMap;

use nom::{
    bytes::complete::tag, character::complete::newline, multi::separated_list1,
    sequence::separated_pair,
};

use crate::Input;

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day20.txt")?;

    println!("20:1 - {}", run_1(&input)?);
    println!("20:2 - {}", run_2(&input)?);

    Ok(())
}

#[derive(Clone)]
enum Pulse {
    High(String, usize),
    Low(String, usize),
}

enum ModType {
    FlipFlop,
    Broadcaster,
    Conjunction,
}

trait Module {
    fn run(&mut self);
    fn add_output(&mut self, o: std::sync::mpsc::Sender<Pulse>);
    // name: String,
    // input: std::sync::mpsc::Receiver<Pulse>,
}

struct Broadcaster {
    input: std::sync::mpsc::Receiver<Pulse>,
    outputs: Vec<std::sync::mpsc::Sender<Pulse>>,
}

impl Module for Broadcaster {
    fn run(&mut self) {
        let mut last_pulse = 0;

        loop {
            let Ok(pulse) = self.input.recv() else {
                return;
            };

            // Send to all outputs
            if self
                .outputs
                .iter()
                .map(|o| o.send(pulse.clone()))
                .find(|r| r.is_err())
                .is_some()
            {
                return;
            }
        }
    }

    fn add_output(&mut self, o: std::sync::mpsc::Sender<Pulse>) {
        self.outputs.push(o);
    }
}

impl Broadcaster {}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let (i, map) = parse(input).map_err(|e| anyhow::anyhow!(e.to_string()))?;
    assert_eq!(i, "");
    dbg!{map};
    todo!()
}

fn run_2(_input: &str) -> anyhow::Result<usize> {
    todo!()
}

fn output_parser(i: crate::Input) -> crate::PResult<Vec<String>> {
    separated_list1(
        tag(", "),
        nom::combinator::map(nom::character::complete::alpha1, |s: &str| s.to_string()),
    )(i)
}
fn type_parser(i: crate::Input) -> crate::PResult<Option<String>> {
    nom::combinator::map(
        nom::combinator::opt(nom::branch::alt((tag("%"), tag("&")))),
        |t| t.map(|t: &str| t.to_string()),
    )(i)
}
fn row_parser(i: Input) -> crate::PResult<(String, (Option<String>, Vec<String>))> {
    let (i, t) = type_parser(i)?;
    let (i, (name, outputs)) =
        separated_pair(nom::character::complete::alpha1, tag(" -> "), output_parser)(i)?;

    Ok((i, (name.to_string(), (t.map(|t| t.to_string()), outputs))))
}

fn parse(i: crate::Input) -> crate::PResult<HashMap<String, (Option<String>, Vec<String>)>> {
    let (i, rows) = separated_list1(newline, row_parser)(i)?;

    Ok((i, rows.into_iter().collect()))
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
    #[test]
    fn day20_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 32000000);
    }

    #[test]
    fn day20_run_2() {}
}
