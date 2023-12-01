pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day1.txt")?;

    println!("1:1 - {}", run_1(&input)?);
    println!("1:2 - {}", run_2(&input)?);

    Ok(())
}

fn run_1(_input: &str) -> anyhow::Result<usize> {
    todo!()
}

fn run_2(_input: &str) -> anyhow::Result<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn aoc1_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 142);
    }

    #[test]
    fn aoc1_run_2() {}
}
