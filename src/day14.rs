pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day14.txt")?;

    println!("14:1 - {}", run_1(&input)?);
    println!("14:2 - {}", run_2(&input)?);

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Rock {
    Rounded,
    Cubic,
}

// fn print_board(board: &[Vec<Option<Rock>>]) {
//     println!("    0123456789");
//     board.iter().enumerate().for_each(|(row_num, row)| {
//         print!("{row_num:2}  ");
//         row.iter().for_each(|o| match o {
//             Some(Rock::Rounded) => print!("O"),
//             Some(Rock::Cubic) => print!("#"),
//             _ => print!("."),
//         });
//         println!();
//     });
// }

fn run_1(input: &str) -> anyhow::Result<usize> {
    let board: Vec<Vec<Option<Rock>>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Some(Rock::Cubic),
                    'O' => Some(Rock::Rounded),
                    _ => None,
                })
                .collect()
        })
        .collect();

    let mut new_board = board.clone();
    for cur_row in 0..board.len() {
        let borrowed_row = new_board[cur_row].clone();
        for (c, o) in borrowed_row.iter().enumerate() {
            // Check below to find ball
            if o.is_none() {
                for row in (cur_row + 1)..board.len() {
                    if Some(Rock::Rounded) == new_board[row][c] {
                        new_board[cur_row][c] = new_board[row][c].take();
                        break;
                    } else if Some(Rock::Cubic) == board[row][c] {
                        break;
                    }
                }
            }
        }
    }

    Ok(new_board
        .iter()
        .rev()
        .enumerate()
        .map(|(r_num, row)| row.iter().filter(|o| *o == &Some(Rock::Rounded)).count() * (r_num + 1))
        .sum())
}

fn run_cycle(board: &mut [Vec<Option<Rock>>]) {
    // North
    for cur_row in 0..board.len() {
        let borrowed_row = board[cur_row].clone();
        for (c, o) in borrowed_row.iter().enumerate() {
            // Check below to find ball
            if o.is_none() {
                for row in (cur_row + 1)..board.len() {
                    if Some(Rock::Rounded) == board[row][c] {
                        board[cur_row][c] = board[row][c].take();
                        break;
                    } else if Some(Rock::Cubic) == board[row][c] {
                        break;
                    }
                }
            }
        }
    }
    // println!("Before West");
    // print_board(&board);

    // West
    for cur_row in 0..board.len() {
        // Check east to find ball
        for col in 0..(board[cur_row].len() - 1) {
            let o = &board[cur_row][col];
            if o.is_none() {
                for check_col in (col + 1)..board[cur_row].len() {
                    if Some(Rock::Rounded) == board[cur_row][check_col] {
                        board[cur_row][col] = board[cur_row][check_col].take();
                        break;
                    } else if Some(Rock::Cubic) == board[cur_row][check_col] {
                        break;
                    }
                }
            }
        }
    }
    // println!("Before South");
    // print_board(&board);

    // South
    for cur_row in (1..board.len()).rev() {
        let borrowed_row = board[cur_row].clone();
        for (c, o) in borrowed_row.iter().enumerate() {
            // Check above to find ball
            if o.is_none() {
                for row in (0..cur_row).rev() {
                    if Some(Rock::Rounded) == board[row][c] {
                        board[cur_row][c] = board[row][c].take();
                        break;
                    } else if Some(Rock::Cubic) == board[row][c] {
                        break;
                    }
                }
            }
        }
    }

    // println!("Before East");
    // print_board(&board);

    // East
    for cur_row in 0..board.len() {
        // Check east to find ball
        for col in (1..(board[cur_row].len())).rev() {
            let o = &board[cur_row][col];
            if o.is_none() {
                for check_col in (0..col).rev() {
                    // println!("col: {col} check: {check_col}");
                    if Some(Rock::Rounded) == board[cur_row][check_col] {
                        board[cur_row][col] = board[cur_row][check_col].take();
                        break;
                    } else if Some(Rock::Cubic) == board[cur_row][check_col] {
                        break;
                    }
                }
            }
        }
    }
}

// too high: 94969
// wrong: 93740
// too low: 93718
// too low: 93694

fn run_2(input: &str) -> anyhow::Result<usize> {
    let mut board: Vec<Vec<Option<Rock>>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Some(Rock::Cubic),
                    'O' => Some(Rock::Rounded),
                    _ => None,
                })
                .collect()
        })
        .collect();

    let mut prev = vec![];
    let dest = 1_000_000_000;
    for i in 1..=dest {
        run_cycle(&mut board);
        if !prev.contains(&board) {
            prev.push(board.clone());
        } else {
            if (i % prev.len()) == (dest % prev.len()) {
                println!("Break {i}");
                println!("m: {}", dest%prev.len());
                break;
            }
        }
    }

    Ok(board
        .iter()
        .rev()
        .enumerate()
        .map(|(r_num, row)| row.iter().filter(|o| *o == &Some(Rock::Rounded)).count() * (r_num + 1))
        .sum())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn day14_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 136);
    }

    #[test]
    fn day14_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 64);
    }
}
