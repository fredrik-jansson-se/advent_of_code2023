pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day3.txt")?;

    println!("3:1 - {}", run_1(&input)?);
    println!("3:2 - {}", run_2(&input)?);

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
    #[test]
    fn day3_run_1() {
    }

    #[test]
    fn day3_run_2() {
    }
}
