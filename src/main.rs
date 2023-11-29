use std::env;

pub type Input<'a> = &'a str;
pub type PResult<'a, T> = nom::IResult<Input<'a>, T, nom::error::VerboseError<Input<'a>>>;

mod aoc1;
mod aoc10;
mod aoc11;
mod aoc12;
mod aoc13;
mod aoc14;
mod aoc15;
mod aoc16;
mod aoc17;
mod aoc18;
mod aoc19;
mod aoc2;
mod aoc20;
mod aoc21;
mod aoc22;
mod aoc23;
mod aoc24;
mod aoc25;
mod aoc3;
mod aoc4;
mod aoc5;
mod aoc6;
mod aoc7;
mod aoc8;
mod aoc9;

fn main() -> anyhow::Result<()> {
    let mut a = env::args();
    a.next();

    let day = a.next().and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);

    match day {
        1 => aoc1::run(),
        2 => aoc2::run(),
        3 => aoc3::run(),
        4 => aoc4::run(),
        5 => aoc5::run(),
        6 => aoc6::run(),
        7 => aoc7::run(),
        8 => aoc8::run(),
        9 => aoc9::run(),
        10 => aoc10::run(),
        11 => aoc11::run(),
        12 => aoc12::run(),
        13 => aoc13::run(),
        14 => aoc14::run(),
        15 => aoc15::run(),
        16 => aoc16::run(),
        17 => aoc17::run(),
        18 => aoc18::run(),
        19 => aoc19::run(),
        20 => aoc20::run(),
        21 => aoc21::run(),
        22 => aoc22::run(),
        23 => aoc23::run(),
        24 => aoc24::run(),
        25 => aoc25::run(),
        _ => Err(anyhow::anyhow!("Not yet implemented")),
    }
}
