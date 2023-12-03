use std::{
    collections::{HashMap, HashSet},
    usize,
};

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day3.txt")?;

    println!("3:1 - {}", run_1(&input)?);
    println!("3:2 - {}", run_2(&input)?);

    Ok(())
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let input: Vec<&str> = input.lines().collect();
    // Find the symbols
    let mut symbol_coords = HashSet::new();
    // Start of digits
    let mut number_start_coords = HashSet::new();

    for (row, line) in input.iter().enumerate() {
        let mut last_was_digit = false;
        for (col, c) in line.chars().enumerate() {
            if c == '.' {
                last_was_digit = false;
            } else if c.is_ascii_digit() {
                if !last_was_digit {
                    number_start_coords.insert((row, col));
                    last_was_digit = true;
                }
            } else {
                last_was_digit = false;
                symbol_coords.insert((row, col));
            }
        }
    }

    let mut res = Vec::new();
    for (row, col) in number_start_coords {
        let end = input[row][col..]
            .chars()
            .position(|c| !c.is_ascii_digit())
            .map(|c| c + col)
            .unwrap_or(input[row].len());
        let num: usize = input[row][col..end].parse()?;
        let start_row = row.max(1) - 1;
        let end_row = row.min(input.len() - 2) + 1;
        #[allow(clippy::needless_range_loop)]
        'outer: for r in start_row..=end_row {
            let start_col = col.max(1) - 1;
            let end_col = end.min(input[r].len() - 2);
            for c in start_col..=end_col {
                if symbol_coords.contains(&(r, c)) {
                    res.push(num);
                    break 'outer;
                }
            }
        }
    }
    Ok(res.iter().sum())
}

// 68166583 -- too low
fn run_2(input: &str) -> anyhow::Result<usize> {
    let input: Vec<&str> = input.lines().collect();
    // Find the symbols
    let mut star_symbol_coords = HashSet::new();
    // Start of digits
    let mut number_start_coords = HashSet::new();

    for (row, line) in input.iter().enumerate() {
        let mut last_was_digit = false;
        for (col, c) in line.chars().enumerate() {
            if c == '.' {
                last_was_digit = false;
            } else if c == '*' {
                star_symbol_coords.insert((row, col));
                last_was_digit = false;
            } else if c.is_ascii_digit() {
                if !last_was_digit {
                    number_start_coords.insert((row, col));
                    last_was_digit = true;
                }
            } else {
                last_was_digit = false;
            }
        }
    }

    let mut res: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    for (row, col) in number_start_coords {
        let end = input[row][col..]
            .chars()
            .position(|c| !c.is_ascii_digit())
            .map(|c| c + col)
            .unwrap_or(input[row].len());
        let num: usize = input[row][col..end].parse()?;
        let start_row = row.max(1) - 1;
        let end_row = row.min(input.len() - 2) + 1;
        #[allow(clippy::needless_range_loop)]
        for r in start_row..=end_row {
            let start_col = col.max(1) - 1;
            let end_col = end.min(input[r].len() - 2);
            for c in start_col..=end_col {
                if star_symbol_coords.contains(&(r, c)) {
                    res.entry((r, c)).or_default().push(num);
                }
            }
        }
    }
    Ok(res
        .values()
        .filter(|v| v.len() == 2)
        .map(|v| v.iter().product::<usize>())
        .sum())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn day3_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 4361);
    }

    #[test]
    fn day3_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 467835);
    }
}
