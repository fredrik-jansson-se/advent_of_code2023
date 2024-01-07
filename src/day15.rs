use nom::InputIter;

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day15.txt")?;

    println!("15:1 - {}", run_1(&input)?);
    println!("15:2 - {}", run_2(&input)?);

    Ok(())
}

fn hash(i: &str) -> usize {
    i.chars().fold(0, |p, c| (17 * (p + (c as usize))) % 256)
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    Ok(input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(hash)
        .sum())
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    const ARRAY_REPEAT_VALUE: Vec<&str> = Vec::new();

    let mut boxes = [ARRAY_REPEAT_VALUE; 256];
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .for_each(|i| {
            if let Some(e) = i.position(|c| c == '=') {
                let label = &i[..e];
                // let d = &i[(e+1)..];
                let h = hash(label);
                if let Some(v) = boxes[h].iter_mut().find(|v| v.starts_with(label)) {
                    *v = i;
                } else {
                    boxes[h].push(i);
                }
            } else if let Some(d) = i.position(|c| c == '-') {
                let label = &i[..d];
                let h = hash(label);
                boxes[h].retain(|v| !v.starts_with(label));
            }
        });

    let focal_len = |i: &str| i.split('=').nth(1).unwrap().parse::<usize>().unwrap();

    Ok(boxes
        .iter()
        .enumerate()
        .map(|(box_idx, b)| {
            b.iter()
                .enumerate()
                .map(|(idx, f)| (box_idx + 1) * (idx + 1) * focal_len(f))
                .sum::<usize>()
        })
        .sum())
}

#[cfg(test)]
mod tests {
    #[test]
    fn day15_hash() {
        assert_eq!(super::hash("HASH"), 52);
        assert_eq!(super::hash("rn=1"), 30);
        assert_eq!(super::hash("rn"), 0);
    }

    #[test]
    fn day15_run_1() {
        assert_eq!(
            super::run_1("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7").unwrap(),
            1320
        );
    }

    #[test]
    fn day15_run_2() {
        assert_eq!(
            super::run_2("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7").unwrap(),
            145
        );
    }
}
