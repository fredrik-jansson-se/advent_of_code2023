pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day9.txt")?;

    println!("9:1 - {}", run_1(&input)?);
    println!("9:2 - {}", run_2(&input)?);

    Ok(())
}

fn calc_next(i: &[isize]) -> Vec<isize> {
    i.windows(2).map(|a| a[1] - a[0]).collect()
}

fn find_bottom(i: &[isize]) -> isize {
    if i.iter().all(|v| *v == 0) {
        return 0;
    }

    let n = calc_next(i);

    let last = find_bottom(&n);
    return last + i.last().unwrap();
}

fn run_1(input: &str) -> anyhow::Result<isize> {
    let histories: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|c| c.parse::<isize>().unwrap())
                .collect()
        })
        .collect();
    Ok(histories
        .iter()
        .map(|v| find_bottom(v))
        .sum())
}

fn run_2(input: &str) -> anyhow::Result<isize> {
    let histories: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|c| c.parse::<isize>().unwrap())
                .rev()
                .collect()
        })
        .collect();
    Ok(histories
        .iter()
        .map(|v| find_bottom(v))
        .sum())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn day9_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 114);
    }

    #[test]
    fn day9_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 2);
    }
}
