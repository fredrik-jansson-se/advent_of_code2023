pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day1.txt")?;

    println!("1:1 - {}", run_1(&input)?);
    println!("1:2 - {}", run_2(&input)?);

    Ok(())
}

fn run_1(input: &str) -> anyhow::Result<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let first: u32 = line
            .chars()
            .find(|c| c.is_ascii_digit())
            .ok_or(anyhow::anyhow!("No digit found"))?
            .to_digit(10)
            .ok_or(anyhow::anyhow!("Can't convert to digit"))?;
        let last: u32 = line
            .chars()
            .rev()
            .find(|c| c.is_ascii_digit())
            .ok_or(anyhow::anyhow!("No digit found"))?
            .to_digit(10)
            .ok_or(anyhow::anyhow!("Can't convert to digit"))?;

        sum += first * 10 + last;
    }
    Ok(sum)
}

fn get_digit(i: &str) -> Option<u32> {
    match i {
        "1" | "one" => Some(1),
        "2" | "two" => Some(2),
        "3" | "three" => Some(3),
        "4" | "four" => Some(4),
        "5" | "five" => Some(5),
        "6" | "six" => Some(6),
        "7" | "seven" => Some(7),
        "8" | "eight" => Some(8),
        "9" | "nine" => Some(9),
        _ => None,
    }
}

fn run_2(input: &str) -> anyhow::Result<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let mut numbers = Vec::new();
        'start: for start_idx in 0..line.len() {
            for end_idx in (start_idx + 1)..=line.len() {
                if let Some(n) = get_digit(&line[start_idx..end_idx]) {
                    numbers.push(n);
                    continue 'start;
                }
            }
        }

        sum += numbers[0] * 10 + numbers[numbers.len() - 1];
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn day1_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 142);
    }

    const INPUT_2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn day1_run_2() {
        assert_eq!(super::run_2(INPUT_2).unwrap(), 281);
    }
}
