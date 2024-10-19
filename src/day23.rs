use crate::common::Coord;

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day23.txt")?;

    println!("23:1 - {}", run_1(&input)?);
    println!("23:2 - {}", run_2(&input)?);

    Ok(())
}

#[derive(PartialEq, Eq)]
enum MapItem {
    Forrest,
    Path,
    SlopeN,
    SlopeE,
    SlopeS,
    SlopeW,
}

type Map = Vec<Vec<MapItem>>;

fn parse(i: &str) -> Map {
    i.lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => MapItem::Path,
                    '#' => MapItem::Forrest,
                    '>' => MapItem::SlopeE,
                    '<' => MapItem::SlopeW,
                    '^' => MapItem::SlopeN,
                    'v' => MapItem::SlopeS,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn can_go(map: &Map, c: Coord) -> bool {
    c.row() < map.len() && c.col() < map[c.row()].len() && map[c.row()][c.col()] != MapItem::Forrest
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let map = parse(input);

    let start_pos: Coord = (0, 1).into();
    let finish_col = map[map.len() - 1]
        .iter()
        .enumerate()
        .find(|(_, i)| **i == MapItem::Path)
        .unwrap()
        .0;

    let finish_pos: Coord = (map.len() - 1, finish_col).into();

    let (longest_path, _cost) = pathfinding::directed::astar::astar(
        &start_pos,
        |pos| {
            println!("successors");
            // return next
            let nbrs: Vec<Coord> = match map[pos.row()][pos.col()] {
                MapItem::Path => pos.neighbors().filter(|n| can_go(&map, *n)).collect(),
                MapItem::SlopeN => [*pos + (-1, 0).into()]
                    .into_iter()
                    .filter(|n| can_go(&map, *n))
                    .collect(),
                MapItem::SlopeE => [*pos + (0, 1).into()]
                    .into_iter()
                    .filter(|n| can_go(&map, *n))
                    .collect(),
                MapItem::SlopeS => [*pos + (1, 0).into()]
                    .into_iter()
                    .filter(|n| can_go(&map, *n))
                    .collect(),
                MapItem::SlopeW => [*pos + (0, -1).into()]
                    .into_iter()
                    .filter(|n| can_go(&map, *n))
                    .collect(),
                MapItem::Forrest => unreachable!(),
            };
            nbrs.into_iter().map(|n| (n, -1isize))
        },
        |pos| pos.manhattan(&finish_pos) as isize,
        |pos| *pos == finish_pos,
    ).unwrap();

    //dbg!{longest_path};
    Ok(longest_path.len())
}

fn run_2(_input: &str) -> anyhow::Result<usize> {
    todo!()
}

#[cfg(test)]
mod tests {

    const INPUT: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    #[ignore]
    fn day23_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 94);
    }

    #[test]
    fn day23_run_2() {}
}
