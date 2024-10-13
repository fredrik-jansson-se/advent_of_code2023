pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day12.txt")?;

    println!("12:1 - {}", run_1(&input)?);
    println!("12:2 - {}", run_2(&input)?);

    Ok(())
}

struct Row {
    springs: Vec<Spring>,
    dmg_groups: Vec<usize>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Spring {
    Damaged,
    Operational,
    Unknown,
}

//fn print_springs(s: &[Spring]) {
//    for s in s {
//        match s {
//            Spring::Damaged => print!("#"),
//            Spring::Operational => print!("."),
//            Spring::Unknown => print!("?"),
//        }
//    }
//}

fn can_be_damaged(s1: &[Spring], len: usize) -> bool {
    if s1.len() < len {
        return false;
    }

    let res = s1
        .iter()
        .take(len)
        .all(|&s| s == Spring::Damaged || s == Spring::Unknown);

    // print_springs(&s1[..len]);
    // println!(" - {len} - {res}");

    res
}

fn find_can_be_damaged(s: &[Spring], len: usize) -> Option<&[Spring]> {
    for i in 0..s.len() {
        if can_be_damaged(&s[i..], len) {
            return Some(&s[(i + len)..]);
        }
    }
    None
}

fn parse(i: crate::Input) -> crate::PResult<Vec<Row>> {
    fn parse_springs(i: crate::Input) -> crate::PResult<Vec<Spring>> {
        let d = nom::combinator::map(nom::bytes::complete::tag("#"), |_| Spring::Damaged);
        let o = nom::combinator::map(nom::bytes::complete::tag("."), |_| Spring::Operational);
        let u = nom::combinator::map(nom::bytes::complete::tag("?"), |_| Spring::Unknown);
        nom::multi::many1(nom::branch::alt((d, o, u)))(i)
    }

    fn parse_dmg_groups(i: crate::Input) -> crate::PResult<Vec<usize>> {
        nom::multi::separated_list1(
            nom::bytes::complete::tag(","),
            nom::combinator::map(nom::character::complete::u64, |v| v as usize),
        )(i)
    }

    fn parse_rows(i: crate::Input) -> crate::PResult<(Vec<Spring>, Vec<usize>)> {
        nom::sequence::separated_pair(
            parse_springs,
            nom::character::complete::space1,
            parse_dmg_groups,
        )(i)
    }

    let (i, rows) = nom::multi::separated_list1(nom::character::complete::newline, parse_rows)(i)?;
    Ok((
        i,
        rows.into_iter()
            .map(|(springs, dmg_groups)| Row {
                springs,
                dmg_groups,
            })
            .collect(),
    ))
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let (i, rows) = parse(input).map_err(|e| anyhow::anyhow!("{e}"))?;
    assert_eq!(i, "");

    let mut results = Vec::new();
    for row in rows.into_iter().skip(1).take(1) {
        let mut combos = 0;
        let max: usize = row.dmg_groups.iter().sum();
        let springs_start = row.springs.as_ptr();
        dbg! {&row.springs};
        dbg! {&row.dmg_groups};
        for start in 0..(row.springs.len() - max) {
            let (_, res) = row.dmg_groups.iter().fold(
                (&row.springs[start..], true),
                |(springs, prev): (&[Spring], bool), grp_len| {
                    //let cur_springs_start = springs.as_ptr();
                    if let Some(next) = find_can_be_damaged(springs, *grp_len) {
                        let offset = unsafe { next.as_ptr().offset_from(springs_start) } - (*grp_len as isize);
                        println!("Found {grp_len} at offset {offset}");
                        if next.len() < 2 {
                            (&[], prev & true)
                        } else {
                            (&next[1..], prev & true)
                        }
                    } else {
                        (&[], false)
                    }
                },
            );
            if res {
                combos += 1;
            }
        }
        results.push(combos);
    }
    dbg! {&results};
    Ok(results.into_iter().sum())
}

fn run_2(_input: &str) -> anyhow::Result<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::Spring;

    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn day12_damage() {
        assert!(super::find_can_be_damaged(&[Spring::Damaged], 1).is_some());
        assert!(super::find_can_be_damaged(&[Spring::Operational, Spring::Damaged], 1).is_some());
        assert!(super::find_can_be_damaged(&[Spring::Operational], 1).is_none());
        assert!(super::find_can_be_damaged(
            &[Spring::Operational, Spring::Unknown, Spring::Damaged],
            2
        )
        .is_some());
        assert!(super::find_can_be_damaged(
            &[Spring::Operational, Spring::Damaged, Spring::Damaged],
            2
        )
        .is_some());
        assert!(super::find_can_be_damaged(
            &[Spring::Operational, Spring::Damaged, Spring::Unknown],
            2
        )
        .is_some());
    }

    #[test]
    fn day12_run_1() {
        //assert_eq!(super::run_1(INPUT).unwrap(), 21);
    }

    #[test]
    fn day12_run_2() {}
}
