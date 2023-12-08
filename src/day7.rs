use std::{cmp::Ordering, collections::HashMap, usize};

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day7.txt")?;

    println!("7:1 - {}", run_1(&input)?);
    println!("7:2 - {}", run_2(&input)?);

    Ok(())
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let mut hands = parse(input);

    hands.sort();

    Ok(hands
        .iter()
        .enumerate()
        .map(|(rank, h)| (rank + 1) * h.bid)
        .sum())
}

// 252829329 - too low
// 252884460 - too low
// 252941412 - too low
fn run_2(input: &str) -> anyhow::Result<usize> {
    let mut hands = parse(input);

    const JOKER: Card = Card(11);
    // First convert jokers to Card(1), to be low worth
    for hand in hands.iter_mut() {
        for card in hand.cards.iter_mut() {
            if *card == JOKER {
                *card = Card(1);
            }
        }
        hand.t = Hand::calculate_type(&hand.cards);
    }

    for hand in hands.iter_mut() {
        let num_jokers = hand.cards.iter().filter(|c| c.0 == 1).count();

        if num_jokers > 0 {
            match hand.t {
                Type::OnePair(c) if num_jokers == 1 => {
                    hand.t = Type::ThreeOfAKind(c);
                }
                Type::ThreeOfAKind(c) if c.0 == 1 && num_jokers == 3 => {
                    // already a joker
                }
                Type::ThreeOfAKind(c) if num_jokers == 1 => {
                    hand.t = Type::FourOfAKind(c);
                }
                Type::FullHouse { three: c1, two: c2 } if c1.0 == 1 || c2.0 == 1 => {
                    hand.t = Type::FiveOfAKind(c2);
                }
                Type::FourOfAKind(c) if num_jokers == 1 => {
                    hand.t = Type::FiveOfAKind(c);
                }
                Type::FourOfAKind(c) if num_jokers == 4 => {
                    hand.t = Type::FiveOfAKind(c);
                }
                Type::TwoPair(c1, _c2) if num_jokers == 2 => {
                    hand.t = Type::FourOfAKind(c1);
                }
                Type::TwoPair(c1, c2) if num_jokers == 1 => {
                    hand.t = Type::FullHouse { three: c1, two: c2 };
                }
                Type::OnePair(c) if c.0 == 1 && num_jokers == 2 => {
                    // already a joker
                }
                Type::FiveOfAKind(_c) if num_jokers == 5 => {
                    // already a joker
                }
                Type::HighCard if num_jokers == 1 => {
                    hand.t = Type::OnePair(Card(1));
                }
                t => {
                    if num_jokers != 0 {
                        dbg! {(t, num_jokers)};
                        // unreachable!()
                    }
                }
            }
        }
    }

    hands.sort();
    // dbg! {&hands};

    Ok(hands
        .iter()
        .enumerate()
        .map(|(rank, h)| (rank + 1) * h.bid)
        .sum())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Card(usize);

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: usize,
    t: Type,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let cmp = self.t.cmp(&other.t);
        if cmp == Ordering::Equal {
            self.cards
                .iter()
                .zip(&other.cards)
                .map(|(c1, c2)| c1.0.cmp(&c2.0))
                .find(|x| *x != Ordering::Equal)
                .unwrap()
        } else {
            cmp
        }
    }
}

impl Hand {
    fn calculate_type(cards: &[Card]) -> Type {
        let mut h: HashMap<Card, usize> = HashMap::new();
        cards.iter().for_each(|c| *h.entry(*c).or_default() += 1);

        // FiveOfAKind
        if let Some(c) = h
            .iter()
            .find_map(|(c, count)| if *count == 5 { Some(*c) } else { None })
        {
            Type::FiveOfAKind(c)
        } else if let Some(c) = h
            .iter()
            .find_map(|(c, count)| if *count == 4 { Some(*c) } else { None })
        {
            Type::FourOfAKind(c)
        } else if let (Some(c1), Some(c2)) = (
            h.iter()
                .find_map(|(c, count)| if *count == 3 { Some(*c) } else { None }),
            h.iter()
                .find_map(|(c, count)| if *count == 2 { Some(*c) } else { None }),
        ) {
            Type::FullHouse { three: c1, two: c2 }
        } else if let Some(c) = h
            .iter()
            .find_map(|(c, count)| if *count == 3 { Some(*c) } else { None })
        {
            Type::ThreeOfAKind(c)
        } else {
            let pairs: Vec<_> = h
                .iter()
                .filter_map(|(c, count)| if *count == 2 { Some(*c) } else { None })
                .collect();
            if pairs.len() == 2 {
                Type::TwoPair(pairs[0], pairs[1])
            } else if pairs.len() == 1 {
                Type::OnePair(pairs[0])
            } else {
                Type::HighCard
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Type {
    FiveOfAKind(Card),
    FourOfAKind(Card),
    FullHouse { three: Card, two: Card },
    ThreeOfAKind(Card),
    TwoPair(Card, Card),
    OnePair(Card),
    HighCard,
}

impl Type {
    fn value(&self) -> usize {
        match self {
            Type::FiveOfAKind(_) => 6,
            Type::FourOfAKind(_) => 5,
            Type::FullHouse { .. } => 4,
            Type::ThreeOfAKind(_) => 3,
            Type::TwoPair(_, _) => 2,
            Type::OnePair(_) => 1,
            Type::HighCard => 0,
        }
    }
}

impl std::cmp::PartialOrd for Type {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Type {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}

fn parse_hand(i: &str) -> Hand {
    let mut hb = i.split(' ');
    let cards: Vec<Card> = hb
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            '2' => Card(2),
            '3' => Card(3),
            '4' => Card(4),
            '5' => Card(5),
            '6' => Card(6),
            '7' => Card(7),
            '8' => Card(8),
            '9' => Card(9),
            'T' => Card(10),
            'J' => Card(11),
            'Q' => Card(12),
            'K' => Card(13),
            'A' => Card(14),
            _ => unreachable!(),
        })
        .collect();

    let bid = hb.next().unwrap().parse().unwrap();
    let t = Hand::calculate_type(&cards);

    Hand { cards, bid, t }
}

fn parse(i: &str) -> Vec<Hand> {
    i.lines().map(parse_hand).collect()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn day7_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 6440);
    }

    #[test]
    fn day7_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 5905);
    }
}
