pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day13.txt")?;

    println!("13:1 - {}", run_1(&input)?);
    println!("13:2 - {}", run_2(&input)?);

    Ok(())
}

// 47527 -- too high
// 47527 -- too high
// 20074 -- too low
// 29463 -- wrong
fn run_1(input: &str) -> anyhow::Result<usize> {
    //let mut maps: Vec<ndarray::ArrayD<usize>> = Vec::new();
    let mut row_maps = Vec::new();
    let mut col_maps: Vec<Vec<usize>> = Vec::new();
    row_maps.push(Vec::new());
    col_maps.push(Vec::new());
    //maps.push(ndarray::ArrayD::default(ndarray::IxDyn(&[0, 0])));
    //let a_zero: ndarray::ArrayD<usize> = ndarray::ArrayD::zeros(ndarray::IxDyn(&[1]));
    //let a_one : ndarray::ArrayD<usize> = ndarray::ArrayD::ones(ndarray::IxDyn(&[1]));
    for line in input.lines() {
        if line.is_empty() {
            //maps.push(ndarray::ArrayD::default(ndarray::IxDyn(&[0, 0])));
            row_maps.push(Vec::new());
            col_maps.push(Vec::new());
            continue;
        }
        //let ml = maps.len();
        //let mut cur_map = &mut maps[ml - 1];
        let mut val = 0;
        //let mut cur_row = ndarray::ArrayD::default(ndarray::IxDyn(&[1]));
        for (col, c) in line.chars().enumerate() {
            if col_maps[col_maps.len() - 1].get(col).is_none() {
                let l = col_maps.len();
                col_maps[l - 1].push(0);
            }

            if c == '#' {
                //dbg!{&cur_row};
                //cur_row.push(ndarray::Axis(0), a_one.view())
                //    .context("add one")?;
                val = (val << 1) + 1;
                let l = col_maps.len();
                let c_val = &mut col_maps[l - 1][col];
                *c_val = (*c_val << 1) + 1;
            } else {
                //dbg!{&cur_row};
                //cur_row.push(ndarray::Axis(0), a_zero.view())?;
                val <<= 1;
                let l = col_maps.len();
                let c_val = &mut col_maps[l - 1][col];
                *c_val <<= 1;
            }
        }

        let l = row_maps.len();
        row_maps[l - 1].push(val);
    }

    assert_eq!(row_maps.len(), col_maps.len());

    let mut res = 0;
    let s = 0;
    for map in col_maps.iter().skip(s) {
        dbg! {map};
        for check_row in 1..(map.len() - 1) {
            let mut above_row = check_row;
            let mut below_row = check_row + 1;
            loop {
                match (map.get(above_row), map.get(below_row)) {
                    (Some(a), Some(b)) if a != b => {
                        break;
                    }
                    (Some(_), Some(_)) => {
                        //
                    }
                    _ => {
                        // we've come to an end
                        println!("Found col {check_row}");
                        res += check_row + 1;
                        break;
                    }
                }
                above_row = above_row.wrapping_sub(1);
                below_row += 1;
            }
        }
    }

    for map in row_maps.iter().skip(s) {
        dbg! {map};
        for check_row in 1..(map.len() - 1) {
            let mut above_row = check_row;
            let mut below_row = check_row + 1;
            loop {
                match (map.get(above_row), map.get(below_row)) {
                    (Some(a), Some(b)) if a != b => {
                        break;
                    }
                    (Some(_), Some(_)) => {
                        //
                    }
                    _ => {
                        // we've come to an end
                        println!("Found row {check_row}");
                        res += 100 * (check_row + 1);
                        break;
                    }
                }
                above_row = above_row.wrapping_sub(1);
                below_row += 1;
            }
        }
    }

    Ok(res)
}

fn run_2(_input: &str) -> anyhow::Result<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn day13_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 405);
    }

    #[test]
    fn day13_run_2() {}
}
