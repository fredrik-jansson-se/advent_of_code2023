use rayon::prelude::*;

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day12.txt")?;

    println!("12:1 - {}", run_1(&input)?);
    println!("12:2 - {}", run_2(&input)?);

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
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

    res
}

fn arrangements(springs: &[Spring], damaged: &[usize]) -> usize {
    if damaged.is_empty() {
        //println!("No more damage groups");
        if !springs.iter().any(|s| *s == Spring::Damaged) {
            return 1;
        } else {
            return 0;
        }
    }
    if springs.is_empty() {
        //println!("Not valid end");
        return 0;
    }

    // Optimization
    if springs.len() < damaged[0] {
        return 0;
    }

    // If next is damaged, we must catch it below
    let tail_cnt = if springs[0] != Spring::Damaged {
        arrangements(&springs[1..], damaged)
    } else {
        0
    };

    //println!("Enter {springs:?} {damaged:?}");
    let d = damaged[0];
    let cnt = if can_be_damaged(springs, d) {
        //println!("Can be damaged {d}");
        let springs_left = &springs[d..];
        // To be valid, either a functional spring or end of springs
        if springs_left.is_empty() {
            arrangements(&[], &damaged[1..])
        } else if springs_left[0] == Spring::Operational || springs_left[0] == Spring::Unknown {
            //println!("Next spring is operational");

            arrangements(&springs_left[1..], &damaged[1..])
        } else {
            0
        }
    } else {
        0
    };
    cnt + tail_cnt
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
    assert!(i.is_empty());

    let mut results = Vec::with_capacity(rows.len());
    for row in rows.into_iter() {
        results.push(arrangements(&row.springs, &row.dmg_groups));
    }
    Ok(results.into_iter().sum())
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let (i, rows) = parse(input).map_err(|e| anyhow::anyhow!("{e}"))?;
    assert!(i.is_empty());

    let results: Vec<usize> = rows
        .par_iter()
        .map(|row| {
            let unfolded = unfold(row);
        println!("row");
            arrangements(&unfolded.springs, &unfolded.dmg_groups)
        })
        .collect();
    //for row in rows.into_iter() {
    //    results.push(arrangements(&unfolded.springs, &unfolded.dmg_groups));
    //    println!("row done");
    //}
    Ok(results.into_iter().sum())
}

fn unfold(r: &Row) -> Row {
    let mut nr = Row {
        springs: Vec::with_capacity((1 + r.springs.len()) * 5),
        dmg_groups: Vec::with_capacity(r.springs.len() * 5),
    };

    for i in 0..5 {
        nr.springs.extend(&r.springs);
        if i < 4 {
            nr.springs.push(Spring::Unknown);
        }
        nr.dmg_groups.extend(&r.dmg_groups);
    }

    nr
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn day12_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 21);
    }

    #[test]
    fn day12_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 525152);
    }

    #[test]
    fn day12_arrangements() {
        let (_, rows) = super::parse("# 1").unwrap();
        assert_eq!(
            super::arrangements(&rows[0].springs, &rows[0].dmg_groups),
            1
        );
        let (_, rows) = super::parse("#.#.### 1,1,3").unwrap();
        assert_eq!(
            super::arrangements(&rows[0].springs, &rows[0].dmg_groups),
            1
        );

        let (_, rows) = super::parse("?.? 1").unwrap();
        assert_eq!(
            super::arrangements(&rows[0].springs, &rows[0].dmg_groups),
            2
        );
        let (_, rows) = super::parse("?#?#?#?#?#?#?#? 1,3,1,6").unwrap();
        assert_eq!(
            super::arrangements(&rows[0].springs, &rows[0].dmg_groups),
            1
        );
        let (_, rows) = super::parse("????.######..#####. 1,6,5").unwrap();
        assert_eq!(
            super::arrangements(&rows[0].springs, &rows[0].dmg_groups),
            4
        );
    }

    #[test]
    //#[ignore]
    fn day12_arrangments_hard() {
        let (_, rows) = super::parse("?###???????? 3,2,1").unwrap();
        assert_eq!(
            super::arrangements(&rows[0].springs, &rows[0].dmg_groups),
            10
        );
    }

    #[test]
    fn day12_unfold() {
        let (_, r1) = super::parse(".# 1").unwrap();
        let (_, r2) = super::parse(".#?.#?.#?.#?.# 1,1,1,1,1").unwrap();
        let r1 = super::unfold(&r1[0]);
        assert_eq!(r1, r2[0]);
    }
}
