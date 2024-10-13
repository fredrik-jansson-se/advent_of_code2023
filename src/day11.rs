use crate::common::Coord;

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day11.txt")?;

    println!("11:1 - {}", run_1(&input)?);
    println!("11:2 - {}", run_2(&input, 1_000_000)?);

    Ok(())
}

fn run_expand(input: &str, expansion: isize) -> anyhow::Result<usize> {
    let mut coords: Vec<Coord> = input
        .lines()
        .enumerate()
        .flat_map(move |(row, line)| {
            line.chars().enumerate().map(move |(col, c)| {
                if c == '#' {
                    Some(Coord::new(row as isize, col as isize))
                } else {
                    None
                }
            })
        })
        .flatten()
        .collect();

    let (mut max_row, mut max_col) = coords.iter().fold((0isize, 0isize), |(r, c), coord| {
        (r.max(coord.irow() + 1), c.max(coord.icol() + 1))
    });

    {
        let mut row = 0;
        while row < max_row {
            // Is this row empty?
            if !coords.iter().any(|c| c.irow() == row) {
                coords.iter_mut().for_each(|c| {
                    if c.irow() > row {
                        c.0 += expansion - 1;
                    }
                });
                row += expansion - 1;
                max_row += expansion - 1;
            }
            row += 1;
        }
    }
    {
        let mut col = 0;
        while col < max_col {
            // Is this col empty?
            if !coords.iter().any(|c| c.icol() == col) {
                coords.iter_mut().for_each(|c| {
                    if c.icol() > col {
                        c.1 += expansion - 1;
                    }
                });
                col += expansion - 1;
                max_col += expansion - 1;
            }
            col += 1;
        }
    }
    let mut dist = 0;
    for (idx, c1) in coords.iter().enumerate() {
        for c2 in coords.iter().skip(idx + 1) {
            dist += c1.manhattan(c2);
        }
    }

    Ok(dist)
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    run_expand(input, 2)
}

fn run_2(input: &str, expansion: isize) -> anyhow::Result<usize> {
    run_expand(input, expansion)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn day11_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 374);
    }

    #[test]
    fn day11_run_2() {
        assert_eq!(super::run_2(INPUT, 10).unwrap(), 1030);
        assert_eq!(super::run_2(INPUT, 100).unwrap(), 8410);
    }
}
