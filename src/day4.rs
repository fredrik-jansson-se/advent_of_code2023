use std::collections::{HashMap, HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{newline, space1},
    combinator::map,
    multi::separated_list1,
};

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day4.txt")?;

    println!("4:1 - {}", run_1(&input)?);
    println!("4:2 - {}", run_2(&input)?);

    Ok(())
}

#[derive(Debug)]
struct Card {
    id: u64,
    winners: HashSet<u64>,
    nums: Vec<u64>,
}

fn parse_card(i: crate::Input) -> crate::PResult<Card> {
    let (i, _) = tag("Card")(i)?;
    let (i, _) = space1(i)?;
    let (i, id) = nom::character::complete::u64(i)?;
    let (i, _) = tag(":")(i)?;
    let (i, _) = space1(i)?;
    let (i, winners) = map(
        separated_list1(space1, nom::character::complete::u64),
        |w| w.into_iter().collect(),
    )(i)?;
    let (i, _) = space1(i)?;
    let (i, _) = tag("|")(i)?;
    let (i, _) = space1(i)?;
    let (i, nums) = separated_list1(space1, nom::character::complete::u64)(i)?;

    Ok((i, Card { id, winners, nums }))
}

fn parse(i: crate::Input) -> crate::PResult<Vec<Card>> {
    separated_list1(newline, parse_card)(i)
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let (_, cards) = parse(input).map_err(|e| anyhow::anyhow!(e.to_string()))?;
    Ok(cards
        .into_iter()
        .map(|card| {
            card.nums
                .iter()
                .fold(0, |score, n| match card.winners.contains(n) {
                    true if score == 0 => 1,
                    true => 2 * score,
                    _ => score,
                })
        })
        .sum())
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let (_, cards) = parse(input).map_err(|e| anyhow::anyhow!(e.to_string()))?;

    let mut multipliers: HashMap<u64, usize> = HashMap::new();
    for card in cards.iter() {
        let num_winners = card
            .nums
            .iter()
            .filter(|num| card.winners.contains(num))
            .count();
        for _ in 0..*multipliers.entry(card.id).or_insert(1) {
            for w in 0..num_winners {
                let winning_id = w as u64 + card.id + 1;
                if winning_id <= cards.len() as u64 {
                    *multipliers.entry(winning_id).or_insert(1) += 1;
                }
            }
        }
    }

    Ok(multipliers
        .iter()
        .filter_map(|m| if (*m.0 as usize) <= cards.len() { Some(m.1) } else { None })
        .sum())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn day4_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 13);
    }

    #[test]
    fn day4_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 30);
    }
}
