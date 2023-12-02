use std::usize;

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day2.txt")?;

    println!("2:1 - {}", run_1(&input)?);
    println!("2:2 - {}", run_2(&input)?);

    Ok(())
}

enum Cube {
    Blue,
    Green,
    Red,
}

struct GameSet {
    blue: usize,
    green: usize,
    red: usize,
}

struct Game {
    id: usize,
    sets: Vec<GameSet>,
}

fn parse_cube(i: crate::Input) -> crate::PResult<Cube> {
    let blue = nom::combinator::map(nom::bytes::complete::tag("blue"), |_| Cube::Blue);
    let green = nom::combinator::map(nom::bytes::complete::tag("green"), |_| Cube::Green);
    let red = nom::combinator::map(nom::bytes::complete::tag("red"), |_| Cube::Red);
    nom::branch::alt((blue, green, red))(i)
}

fn parse_cube_num(i: crate::Input) -> crate::PResult<(usize, Cube)> {
    let (i, num) = nom::combinator::map(nom::character::complete::u32, |v| v as usize)(i)?;
    let (i, _) = nom::character::complete::space1(i)?;
    let (i, cube) = parse_cube(i)?;
    Ok((i, (num, cube)))
}

fn parse_set(i: crate::Input) -> crate::PResult<GameSet> {
    let (i, set) = nom::multi::separated_list1(nom::bytes::complete::tag(", "), parse_cube_num)(i)?;

    let game_set = set.iter().fold(
        GameSet {
            blue: 0,
            green: 0,
            red: 0,
        },
        |set, cube_num| match cube_num {
            (num, Cube::Blue) => GameSet { blue: *num, ..set },
            (num, Cube::Red) => GameSet { red: *num, ..set },
            (num, Cube::Green) => GameSet { green: *num, ..set },
        },
    );

    Ok((i, game_set))
}

fn parse_game(i: crate::Input) -> crate::PResult<Game> {
    let (i, _) = nom::bytes::complete::tag("Game ")(i)?;
    let (i, id) = nom::character::complete::u32(i)?;
    let (i, _) = nom::bytes::complete::tag(": ")(i)?;
    let (i, sets) = nom::multi::separated_list1(nom::bytes::complete::tag("; "), parse_set)(i)?;
    Ok((i, Game { id: id as _, sets }))
}

fn parse(i: crate::Input) -> crate::PResult<Vec<Game>> {
    nom::multi::separated_list1(nom::character::complete::newline, parse_game)(i)
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let (_, games) = parse(input).map_err(|e| anyhow::anyhow!(e.to_string()))?;
    let check = GameSet {
        red: 12,
        green: 13,
        blue: 14,
    };

    Ok(games
        .iter()
        .filter(|game| {
            game.sets.iter().all(|set| {
                set.red <= check.red && set.green <= check.green && set.blue <= check.blue
            })
        })
        .map(|g| g.id)
        .sum())
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let (_, games) = parse(input).map_err(|e| anyhow::anyhow!(e.to_string()))?;
    Ok(games
        .iter()
        .map(|game| {
            game.sets.iter().fold(
                GameSet {
                    red: 0,
                    blue: 0,
                    green: 0,
                },
                |set_a, set_b| GameSet {
                    red: set_a.red.max(set_b.red),
                    green: set_a.green.max(set_b.green),
                    blue: set_a.blue.max(set_b.blue),
                },
            )
        })
        .map(|set| set.red * set.green * set.blue)
        .sum())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn day2_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 8);
    }

    #[test]
    fn day2_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 2286)
    }
}
