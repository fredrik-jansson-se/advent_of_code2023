use std::usize;

use nom::{
    bytes::complete::tag,
    character::complete::{newline, space1},
    multi::separated_list1,
};

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day6.txt")?;

    println!("6:1 - {}", run_1(&input)?);
    println!("6:2 - {}", run_2(&input)?);

    Ok(())
}

#[derive(Debug)]
struct Race {
    race_time: usize,
    record_dist: usize,
}

// Tp = time pressed
// v = Tp
// T = time traveling = race time - T
// S = Tp * (race time - Tp)
//
// dS = S - record dist, find dS > 0
//
// Find zero of:
// Tp * ( race time - Tp) - record_dist = 0 <=>
// -Tp^2 + race_time*Tp - record_dist = 0

// Brute force worked....
// impl Race {
//     // Only interested in positive values
//     fn solve_quadratic(&self) -> Vec<f32> {
//         let a = -1.0;
//         let b = (self.race_time as f32);
//         let c = -(self.record_dist as f32);
//         let mut s1 = (-b - (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
//         let mut s2 = (-b - (b * b + 4.0 * a * c).sqrt()) / (2.0 * a);
//         if s2 < s1 {
//             std::mem::swap(&mut s1, &mut s2);
//         }
//         if s1 >= 0.0 {
//             vec![s1, s2]
//         } else {
//             vec![s2]
//         }
//     }
// }

fn parse(i: crate::Input) -> crate::PResult<Vec<Race>> {
    let (i, _) = tag("Time:")(i)?;
    let (i, _) = space1(i)?;
    let (i, times) = separated_list1(space1, nom::character::complete::u32)(i)?;
    let (i, _) = newline(i)?;
    let (i, _) = tag("Distance:")(i)?;
    let (i, _) = space1(i)?;
    let (i, distances) = separated_list1(space1, nom::character::complete::u32)(i)?;
    let (i, _) = nom::combinator::opt(newline)(i)?;

    Ok((
        i,
        times
            .into_iter()
            .zip(distances)
            .map(|(t, d)| Race {
                race_time: t as usize,
                record_dist: d as usize,
            })
            .collect(),
    ))
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let (_, races) = parse(input).map_err(|e| anyhow::anyhow!(e.to_string()))?;
    let mut res = Vec::new();

    for race in races {
        let races_won = (0..=race.race_time)
            .map(|tp| tp * (race.race_time - tp))
            .filter(|s| *s > race.record_dist)
            .count();
        res.push(races_won);
    }

    Ok(res.iter().product())
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let (_, races) = parse(input).map_err(|e| anyhow::anyhow!(e.to_string()))?;

    let race = Race {
        race_time: races
            .iter()
            .map(|r| r.race_time.to_string())
            .collect::<Vec<_>>()
            .join("")
            .parse()?,
        record_dist: races
            .iter()
            .map(|r| r.record_dist.to_string())
            .collect::<Vec<_>>()
            .join("")
            .parse()?,
    };

    let races_won = (0..=race.race_time)
        .map(|tp| tp * (race.race_time - tp))
        .filter(|s| *s > race.record_dist)
        .count();

    Ok(races_won)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn day6_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 288);
    }

    #[test]
    fn day6_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 71503);
    }
}
