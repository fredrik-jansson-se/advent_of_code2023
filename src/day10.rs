use std::collections::HashSet;

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day10.txt")?;

    println!("10:1 - {}", run_1(&input)?);
    println!("10:2 - {}", run_2(&input)?);

    Ok(())
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let map: Map = input.lines().map(|row| row.chars().collect()).collect();

    let start: Coord = map
        .iter()
        .enumerate()
        .find_map(|(r, row)| {
            row.iter()
                .enumerate()
                .find(|(_, c)| **c == 'S')
                .map(|(col, _)| (r, col).into())
        })
        .unwrap();

    let mut to_search: Vec<Coord> = Vec::new();
    let mut visited: HashSet<Coord> = HashSet::new();

    to_search.push(start);

    while let Some(cur) = to_search.pop() {
        if visited.contains(&cur) {
            continue;
        }
        visited.insert(cur);

        let nbrs = cur
            .neighbors()
            .filter(|coord| !visited.contains(coord) && can_go(&cur, coord, &map));

        to_search.extend(nbrs);
    }

    Ok(visited.len() / 2)
}

fn run_2(_input: &str) -> anyhow::Result<usize> {
    todo!()
}

type Map = Vec<Vec<char>>;
type Coord = crate::common::Coord;

fn can_go(from: &Coord, to: &Coord, map: &Map) -> bool {
    if map
        .get(to.row())
        .and_then(|row| row.get(to.col()))
        .is_none()
    {
        return false;
    }
    let src = map[from.row()][from.col()];
    match (to.irow() - from.irow(), to.icol() - from.icol()) {
        // Coming from north
        (1, 0) => {
            (src == '|' || src == '7' || src == 'F' || src == 'S')
                && matches!(map[to.row()][to.col()], '|' | 'L' | 'J')
        }
        // Coming from south
        (-1, 0) => {
            (src == '|' || src == 'L' || src == 'J' || src == 'S')
                && matches!(map[to.row()][to.col()], '|' | '7' | 'F')
        }
        // coming from west
        (0, 1) => {
            (src == '-' || src == 'L' || src == 'F' || src == 'S')
                && matches!(map[to.row()][to.col()], '-' | 'J' | '7')
        }
        // coming from east
        (0, -1) => (src == '-' || src == 'J' || src=='7' || src=='S') &&
            matches!(map[to.row()][to.col()], '-' | 'L' | 'F'),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    const INPUT_1: &str = ".....
.S-7.
.|.|.
.L-J.
.....";
    const INPUT_2: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    #[test]
    fn day10_run_1() {
        assert_eq!(super::run_1(INPUT_1).unwrap(), 4);
        assert_eq!(super::run_1(INPUT_2).unwrap(), 8);
    }

    #[test]
    fn day10_run_2() {}
}
