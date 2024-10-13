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
    High { pulse_idx: usize },
    Low { pulse_idx: usize },
}

// impl Ord for Pulse {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         let s_pulse_idx = match self {
//             Pulse::High { pulse_idx } => pulse_idx,
//             Pulse::Low { pulse_idx } => pulse_idx,
//         };

//         let o_pulse_idx = match other {
//             Pulse::High { pulse_idx } => pulse_idx,
//             Pulse::Low { pulse_idx } => pulse_idx,
//         };
    
//         s_pulse_idx.cmp(o_pulse_idx)
//     }
// }

#[derive(Debug)]
enum ModType {
    FlipFlop,
    Broadcaster,
    Conjunction,
}

trait Module {
    fn add_output(&mut self, o: &str);
    fn inc_inputs(&mut self);
    fn set_input(&mut self, from: &str, pulse: &Pulse) -> Vec<(String, Pulse)>;
}

struct Broadcaster {
    outputs: Vec<String>,
}

impl Module for Broadcaster {
    // fn run(&mut self) -> Vec<Pulse> {
    //     let pulse = Pulse::Low("broadcast".to_string(), self.pulse_idx);
    //     self.pulse_idx+=1;
    //     self.outputs.iter().map(|o| Pulse::Low(o.to_string(), self.pulse_idx)).collect()
    // }

    fn add_output(&mut self, o: &str) {
        self.outputs.push(o.to_string());
    }

    fn inc_inputs(&mut self) {}

    fn set_input(&mut self, _from: &str, pulse: &Pulse) -> Vec<(String, Pulse)> {
        self.outputs
            .iter()
            .map(|_o| ("broadcast".to_string(), pulse.clone()))
            .collect()
    }
}

impl Broadcaster {
    fn new() -> Self {
        Self {
            outputs: Vec::new(),
        }
    }
}

struct FlipFlop {
    on: bool,
    num_inputs: usize,
    outputs: Vec<String>,
}

impl Module for FlipFlop {
    fn add_output(&mut self, o: &str) {
        self.outputs.push(o.to_string());
    }

    fn inc_inputs(&mut self) {
        self.num_inputs += 1;
    }

    fn set_input(&mut self, _from: &str, pulse: &Pulse) -> Vec<(String, Pulse)> {
        if let Pulse::Low { pulse_idx } = pulse {
            let new_pulse = if self.on {
                self.on = false;
                Pulse::Low {
                    pulse_idx: pulse_idx + 1,
                }
            } else {
                self.on = true;
                Pulse::High {
                    pulse_idx: pulse_idx + 1,
                }
            };
            return self
                .outputs
                .iter()
                .map(|o| (o.to_string(), new_pulse.clone()))
                .collect();
        }
        Vec::new()
    }
}

impl FlipFlop {
    fn new() -> Self {
        Self {
            on: false,
            num_inputs: 0,
            outputs: Vec::new(),
        }
    }
}

struct Conjunction {
    last_inputs: HashMap<String, Pulse>,
    num_inputs: usize,
    outputs: Vec<String>,
}

impl Module for Conjunction {
    fn add_output(&mut self, o: &str) {
        self.outputs.push(o.to_string());
    }

    fn inc_inputs(&mut self) {
        self.num_inputs += 1;
    }

    fn set_input(&mut self, _from: &str, _pulse: &Pulse) -> Vec<(String, Pulse)> {
        //let last = self
        //    .last_inputs
        //    .entry(from.to_string())
        //    .or_insert(Pulse::Low { pulse_idx: 0 });
        // if
        todo!()
    }
}

impl Conjunction {
    fn new() -> Self {
        Self {
            last_inputs: HashMap::new(),
            num_inputs: 0,
            outputs: Vec::new(),
        }
    }
}

fn run_simulation(_models: &mut HashMap<String, Box<dyn Module>>, iters: usize) {
    for _ in 0..iters {
        // let mut heap = BinaryHeap::new();
        // let
    }
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let (_i, mut map) = parse(input).map_err(|e| anyhow::anyhow!(e.to_string()))?;
    run_simulation(&mut map, 1000);
    todo!();
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

    for m in rows.iter() {
        let name = &m.0;
        match m.1 .0 {
            ModType::FlipFlop => {
                res.insert(name.to_string(), Box::new(FlipFlop::new()));
            }
            ModType::Broadcaster => {
                res.insert(name.to_string(), Box::new(Broadcaster::new()));
            }
            ModType::Conjunction => {
                res.insert(name.to_string(), Box::new(Conjunction::new()));
            }
        }
    }

    for m in rows {
        {
            let the_mod = res.get_mut(&m.0).unwrap();
            for output in m.1 .1.iter() {
                the_mod.add_output(&output);
            }
        }
        for output in m.1 .1.iter() {
            res.get_mut(output).unwrap().inc_inputs();
        }
    }

    Ok((i, res))
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
        //assert_eq!(super::run_1(INPUT).unwrap(), 32000000);
    }

    #[test]
    fn day20_run_2() {}
}
