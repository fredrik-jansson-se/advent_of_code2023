use std::collections::{HashMap, VecDeque};

use crate::Input;
use nom::{
    bytes::complete::tag, character::complete::newline, multi::separated_list1,
    sequence::separated_pair,
};

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day20.txt")?;

    println!("20:1 - {}", run_1(&input)?);
    println!("20:2 - {}", run_2(&input)?);

    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug)]
enum ModType {
    FlipFlop,
    Broadcaster,
    Conjunction,
}

trait Module: std::fmt::Debug {
    fn add_input(&mut self, i: &str);
    fn set_input(
        &mut self,
        from: &str,
        to: &str,
        pulse: Pulse,
    ) -> VecDeque<(String, String, Pulse)>;
    fn get_outputs(&self) -> Vec<String>;
}

#[derive(Debug)]
struct Broadcaster {
    outputs: Vec<String>,
}

impl Module for Broadcaster {
    fn add_input(&mut self, _o: &str) {
        unreachable!()
    }

    fn set_input(
        &mut self,
        _from: &str,
        to: &str,
        pulse: Pulse,
    ) -> VecDeque<(String, String, Pulse)> {
        self.outputs
            .iter()
            .map(|o| (to.to_string(), o.to_string(), pulse))
            .collect()
    }

    fn get_outputs(&self) -> Vec<String> {
        self.outputs.clone()
    }
}

impl Broadcaster {
    fn new(outputs: &[String]) -> Self {
        Self {
            outputs: outputs.to_vec(),
        }
    }
}

#[derive(Debug)]
struct FlipFlop {
    on: bool,
    outputs: Vec<String>,
}

impl Module for FlipFlop {
    fn add_input(&mut self, _i: &str) {
    }

    fn set_input(
        &mut self,
        _from: &str,
        to: &str,
        pulse: Pulse,
    ) -> VecDeque<(String, String, Pulse)> {
        if Pulse::Low == pulse {
            self.on = !self.on;
            let new_pulse = if self.on { Pulse::High } else { Pulse::Low };

            self.outputs
                .iter()
                .map(|o| (to.to_string(), o.to_string(), new_pulse))
                .collect()
        } else {
            VecDeque::new()
        }
    }
    fn get_outputs(&self) -> Vec<String> {
        self.outputs.clone()
    }
}

impl FlipFlop {
    fn new(outputs: &[String]) -> Self {
        Self {
            on: false,
            outputs: outputs.to_vec(),
        }
    }
}

#[derive(Debug)]
struct Conjunction {
    inputs: HashMap<String, Pulse>,
    outputs: Vec<String>,
}

impl Module for Conjunction {
    fn add_input(&mut self, o: &str) {
        self.inputs.insert(o.to_string(), Pulse::Low);
    }

    fn set_input(
        &mut self,
        from: &str,
        to: &str,
        pulse: Pulse,
    ) -> VecDeque<(String, String, Pulse)> {
        *self.inputs.get_mut(from).unwrap() = pulse;
        let all_high = self.inputs.values().all(|p| *p == Pulse::High);
        let new_pulse = if all_high { Pulse::Low } else { Pulse::High };
        self.outputs
            .iter()
            .map(|o| (to.to_string(), o.to_string(), new_pulse))
            .collect()
    }
    fn get_outputs(&self) -> Vec<String> {
        self.outputs.clone()
    }
}

impl Conjunction {
    fn new(outputs: &[String]) -> Self {
        Self {
            inputs: Default::default(),
            outputs: outputs.to_vec(),
        }
    }
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let (_i, mut map) = parse(input).map_err(|e| anyhow::anyhow!(e.to_string()))?;
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    for _ in 0..1000 {
        // Add low pulse from button
        low_pulses += 1;
        let bcast = "broadcaster";

        let mut signals_to_process =
            map.get_mut(bcast)
                .unwrap()
                .set_input("button", bcast, Pulse::Low);

        while let Some((from, to, pulse)) = signals_to_process.pop_front() {
            if pulse == Pulse::Low {
                low_pulses += 1;
            } else {
                high_pulses += 1;
            };
            //println!("{from} -{pulse:?} -> {to}");
            if let Some(module) = map.get_mut(&to) {
                let mut new_signals = module.set_input(&from, &to, pulse);
                //new_signals.append(&mut signals_to_process);
                signals_to_process.append(&mut new_signals);
            }
        }
    }
    Ok(low_pulses * high_pulses)
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
fn type_parser(i: crate::Input) -> crate::PResult<ModType> {
    let flip_flop = nom::combinator::map(tag("%"), |_| ModType::FlipFlop);
    let conj = nom::combinator::map(tag("&"), |_| ModType::Conjunction);

    let (i, r) = nom::combinator::opt(nom::branch::alt((flip_flop, conj)))(i)?;

    if let Some(mt) = r {
        Ok((i, mt))
    } else {
        Ok((i, ModType::Broadcaster))
    }
}
fn row_parser(i: Input) -> crate::PResult<(String, (ModType, Vec<String>))> {
    let (i, t) = type_parser(i)?;
    let (i, (name, outputs)) =
        separated_pair(nom::character::complete::alpha1, tag(" -> "), output_parser)(i)?;

    Ok((i, (name.to_string(), (t, outputs))))
}

fn parse(i: crate::Input) -> crate::PResult<HashMap<String, Box<dyn Module>>> {
    let (i, rows) = separated_list1(newline, row_parser)(i)?;

    let mut res: HashMap<String, Box<dyn Module>> = HashMap::new();

    for (name, (t, outputs)) in rows.iter() {
        match t {
            ModType::FlipFlop => {
                res.insert(name.to_string(), Box::new(FlipFlop::new(outputs)));
            }
            ModType::Broadcaster => {
                res.insert(name.to_string(), Box::new(Broadcaster::new(outputs)));
            }
            ModType::Conjunction => {
                res.insert(name.to_string(), Box::new(Conjunction::new(outputs)));
            }
        }
    }

    // Update the inputs
    let names: Vec<_> = res.keys().cloned().collect();
    for name in names {
        let outputs = res[&name].get_outputs();
        for output in outputs {
            if let Some(output) = res.get_mut(&output) {
                output.add_input(&name);
            }
        }
    }

    Ok((i, res))
}

#[cfg(test)]
mod tests {
    const INPUT_1: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const INPUT_2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn day20_run_1() {
        assert_eq!(super::run_1(INPUT_1).unwrap(), 32000000);
        assert_eq!(super::run_1(INPUT_2).unwrap(), 11687500);
    }

    #[test]
    fn day20_run_2() {}
}
