pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day14.txt")?;

    println!("14:1 - {}", run_1(&input)?);
    println!("14:2 - {}", run_2(&input)?);

    Ok(())
}

#[derive(PartialEq, Clone)]
enum Obj {
    Rock,
    Stone,
}

// 107003 -- too low
fn run_1(input: &str) -> anyhow::Result<usize> {
    let board: Vec<Vec<Option<Obj>>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Some(Obj::Rock),
                    'O' => Some(Obj::Stone),
                    _ => None,
                })
                .collect()
        })
        .collect();

    let mut new_board = board.clone();
    for cur_row in 0..board.len() {
        for (c, o) in board[cur_row].iter().enumerate() {
            // Check below to find ball
            if o.is_none() {
                for row in (cur_row + 1)..board.len() {
                    if Some(Obj::Stone) == board[row][c] {
                        new_board[cur_row][c] = new_board[row][c].take();
                        break;
                    } else if Some(Obj::Rock) == board[row][c] {
                        break;
                    }
                }
            }
        }
    }
    // board.iter().for_each(|row| {
    //     row.iter().for_each(|o| match o {
    //         Some(Obj::Stone) => print!("O"),
    //         Some(Obj::Rock) => print!("#"),
    //         _ => print!(" "),
    //     });
    //     println!();
    // });
    // println!();
    // new_board.iter().for_each(|row| {
    //     row.iter().for_each(|o| match o {
    //         Some(Obj::Stone) => print!("O"),
    //         Some(Obj::Rock) => print!("#"),
    //         _ => print!(" "),
    //     });
    //     println!();
    // });

    Ok(new_board
        .iter()
        .rev()
        .enumerate()
        .map(|(r_num, row)| row.iter().filter(|o| *o == &Some(Obj::Stone)).count() * (r_num + 1))
        .sum())
}

fn run_2(_input: &str) -> anyhow::Result<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";
    #[test]
    fn day14_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 136);
    }

    #[test]
    fn day14_run_2() {}
}
