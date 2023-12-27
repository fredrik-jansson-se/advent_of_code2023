use crate::common::Dir;

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day17.txt")?;

    println!("17:1 - {}", run_1(&input)?);
    println!("17:2 - {}", run_2(&input)?);

    Ok(())
}


type Coord = (isize, isize);

#[derive(Clone,Hash)]
struct Pos(crate::common::Pos);

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.0.c == other.0.c
    }
}

impl Eq for Pos {}

impl AsRef<crate::common::Pos> for Pos {
    fn as_ref(&self) -> &crate::common::Pos {
        &self.0
    }
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let map: Vec<Vec<isize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as isize)
                .collect()
        })
        .collect();

    let sp = (
        Pos(crate::common::Pos {
            c: (0, 0).into(),
            dir: Dir::E,
        }),
        // std::collections::VecDeque::new(),
    );

    let finish = 
        crate::common::Pos {
            c: 
        ((map.len() - 1) as isize,
        (map[map.len() - 1].len() - 1) as isize).into(),
        dir: crate::common::Dir::S,
        };

    // let path = pathfinding::directed::astar::astar(
    //     &sp,
    //     |(cur_pos, prev_visited)| {
    //         let mut res: Vec<Pos> = Vec::with_capacity(3);
    //         let mut new_prev_visited = prev_visited.clone();
    //         new_prev_visited.push_back(cur_pos.clone());
    //         if prev_visited.len() == 3 {
    //             new_prev_visited.remove(0);
    //             if prev_visited[0].0.manhattan(&cur_pos.0) < 3 {
    //                 res.push(Pos(cur_pos.0.move_forward()));
    //             }
    //         }
    //         let res: Vec<(Pos, std::collections::VecDeque<Pos>)> = res
    //             .into_iter()
    //             .map(|r: Pos| (r, new_prev_visited.clone()))
    //             .collect();
    //         res
    //     },
    //     |(p, _)| p.0.manhattan(&finish),
    //     |(p, _)| p.0.c == finish.c,
    // );
    todo!()
}

fn run_2(_input: &str) -> anyhow::Result<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn day17_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 102);
    }

    #[test]
    fn day17_run_2() {}
}
